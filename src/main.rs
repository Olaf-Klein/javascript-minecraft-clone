mod input;
mod renderer;
mod settings;
mod world;

use glam::Vec3;
use input::InputState;
use renderer::{Camera, Renderer};
use settings::GameSettings;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};
use world::World;

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    world: World,
    camera: Camera,
    input: InputState,
    settings: GameSettings,
    last_frame: Instant,
    delta_time: Duration,
}

impl App {
    fn new() -> Self {
        let settings = GameSettings::load();
        let world = World::new(None);
        let camera = Camera::new(Vec3::new(8.0, 80.0, 8.0), 1.0);

        Self {
            window: None,
            renderer: None,
            world,
            camera,
            input: InputState::new(),
            settings,
            last_frame: Instant::now(),
            delta_time: Duration::from_millis(16),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;

        let dt = self.delta_time.as_secs_f32();
        let speed = if self.input.is_down() {
            50.0 * dt
        } else {
            20.0 * dt
        };

        // Update camera position based on input
        let front = self.camera.get_front();
        let right = self.camera.get_right();

        if self.input.is_forward() {
            self.camera.position += front * speed;
        }
        if self.input.is_backward() {
            self.camera.position -= front * speed;
        }
        if self.input.is_left() {
            self.camera.position -= right * speed;
        }
        if self.input.is_right() {
            self.camera.position += right * speed;
        }
        if self.input.is_up() {
            self.camera.position.y += speed;
        }
        if self.input.is_down() && !self.input.is_key_pressed(KeyCode::ShiftLeft) {
            self.camera.position.y -= speed;
        }

        // Update camera rotation based on mouse
        if self.input.is_mouse_captured() {
            let (dx, dy) = self.input.get_mouse_delta();
            self.camera.yaw += dx as f32 * self.settings.mouse_sensitivity;
            self.camera.pitch -= dy as f32 * self.settings.mouse_sensitivity;
            self.camera.pitch = self.camera.pitch.clamp(-89.0, 89.0);
            self.input.reset_mouse_delta();
        }

        // Load chunks around player
        let player_chunk_x = (self.camera.position.x / world::CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_z = (self.camera.position.z / world::CHUNK_SIZE as f32).floor() as i32;
        let render_distance = self.settings.graphics.render_distance as i32;

        for x in (player_chunk_x - render_distance)..=(player_chunk_x + render_distance) {
            for z in (player_chunk_z - render_distance)..=(player_chunk_z + render_distance) {
                let chunk = self.world.get_chunk(x, z);
                if let Some(renderer) = &mut self.renderer {
                    renderer.generate_chunk_mesh(chunk);
                }
            }
        }

        // Update renderer
        if let Some(renderer) = &mut self.renderer {
            renderer.update_camera(&self.camera);
            renderer.update_chunks(&self.world, self.camera.position);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_title("Minecraft Clone - Rust")
                .with_inner_size(winit::dpi::LogicalSize::new(1200, 800));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            
            // Set up cursor grab
            let _ = window.set_cursor_grab(CursorGrabMode::Confined);
            window.set_cursor_visible(false);
            self.input.set_mouse_captured(true);

            // Update camera aspect ratio
            let size = window.inner_size();
            self.camera
                .update_aspect(size.width as f32 / size.height as f32);

            // Create renderer
            let renderer = pollster::block_on(Renderer::new(window.clone())).unwrap();

            self.renderer = Some(renderer);
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                    self.camera.update_aspect(
                        physical_size.width as f32 / physical_size.height as f32,
                    );
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        self.input.key_pressed(key);
                        if key == KeyCode::Escape {
                            if self.input.is_mouse_captured() {
                                if let Some(window) = &self.window {
                                    let _ = window.set_cursor_grab(CursorGrabMode::None);
                                    window.set_cursor_visible(true);
                                    self.input.set_mouse_captured(false);
                                }
                            } else {
                                event_loop.exit();
                            }
                        }
                    }
                    ElementState::Released => {
                        self.input.key_released(key);
                    }
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if !self.input.is_mouse_captured() {
                    if let Some(window) = &self.window {
                        let _ = window.set_cursor_grab(CursorGrabMode::Confined);
                        window.set_cursor_visible(false);
                        self.input.set_mouse_captured(true);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.update();

                if let Some(renderer) = &mut self.renderer {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            if let Some(window) = &self.window {
                                renderer.resize(window.inner_size());
                            }
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            event_loop.exit();
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if self.input.is_mouse_captured() {
                self.input.update_mouse_delta(delta);
            }
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("Starting Minecraft Clone - Rust Edition");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
