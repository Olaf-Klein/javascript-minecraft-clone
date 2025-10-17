mod input;
mod renderer;
mod ui;
mod settings;
mod world;

use glam::{IVec3, Vec3};
use input::InputState;
use renderer::{Camera, Renderer};
use ui::Gui;
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
use world::{BlockType, World, CHUNK_SIZE, WORLD_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppScreen {
    MainMenu,
    Worlds,
    Playing,
    Paused,
    Settings,
}

struct BlockHit {
    hit: IVec3,
    place: Option<IVec3>,
}

struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    gui: Option<Gui>,
    world: World,
    screen: AppScreen,
    worlds_dir: std::path::PathBuf,
    /// Previous screen used to return from settings
    screen_prev: Option<AppScreen>,
    /// Temporary buffer for creating new worlds
    new_world_name: String,
    /// Pending delete confirmation for a world path
    confirm_delete: Option<std::path::PathBuf>,
    selected_block: BlockType,
    camera: Camera,
    input: InputState,
    settings: GameSettings,
    last_frame: Instant,
    delta_time: Duration,
}

impl App {
    fn new(worlds_dir: std::path::PathBuf) -> Self {
        let settings = GameSettings::load();
        let world = World::new(None, None);
        let mut camera = Camera::new(Vec3::new(8.0, 80.0, 8.0), 1.0);
        camera.fov = settings.graphics.fov;

        Self {
            window: None,
            renderer: None,
            gui: None,
            world,
            screen: AppScreen::MainMenu,
            worlds_dir,
            screen_prev: None,
            new_world_name: String::new(),
            confirm_delete: None,
            selected_block: BlockType::GrassBlock,
            camera,
            input: InputState::new(),
            settings,
            last_frame: Instant::now(),
            delta_time: Duration::from_millis(16),
        }
    }

    fn apply_cursor_capture(window: &Window, captured: bool) {
        let _ = window.set_cursor_grab(if captured {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        });
        window.set_cursor_visible(!captured);
    }

    fn set_cursor_capture(&mut self, captured: bool) {
        self.input.set_mouse_captured(captured);
        if let Some(window) = &self.window {
            Self::apply_cursor_capture(window.as_ref(), captured);
        }
    }

    fn pick_block(&mut self, max_distance: f32) -> Option<BlockHit> {
        let origin = self.camera.position;
        let direction = self.camera.get_front().normalize();
        let mut last_empty: Option<IVec3> = None;
        let mut last_checked: Option<IVec3> = None;
        let mut t = 0.0;
        let step = 0.1;

        while t <= max_distance {
            let sample = origin + direction * t;
            let block_pos = IVec3::new(
                sample.x.floor() as i32,
                sample.y.floor() as i32,
                sample.z.floor() as i32,
            );

            if last_checked != Some(block_pos) {
                last_checked = Some(block_pos);
                if block_pos.y >= 0 && block_pos.y < WORLD_HEIGHT as i32 {
                    let block = self
                        .world
                        .get_block_at(block_pos.x, block_pos.y, block_pos.z);
                    if block.is_solid() {
                        return Some(BlockHit {
                            hit: block_pos,
                            place: last_empty,
                        });
                    } else {
                        last_empty = Some(block_pos);
                    }
                } else {
                    last_empty = Some(block_pos);
                }
            }

            t += step;
        }

        None
    }

    fn break_block(&mut self) {
        if let Some(hit) = self.pick_block(8.0) {
            if hit.hit.y < 0 || hit.hit.y >= WORLD_HEIGHT as i32 {
                return;
            }

            let block = self
                .world
                .get_block_at(hit.hit.x, hit.hit.y, hit.hit.z);
            if block.hardness() < 0.0 {
                return;
            }

            if let Some((chunk_x, chunk_z)) = self.world.set_block_at(
                hit.hit.x,
                hit.hit.y,
                hit.hit.z,
                BlockType::Air,
            ) {
                self.invalidate_chunk_and_neighbors(chunk_x, chunk_z, hit.hit);
            }
        }
    }

    fn place_block(&mut self) {
        if let Some(hit) = self.pick_block(8.0) {
            let Some(place_pos) = hit.place else {
                return;
            };

            if place_pos.y < 0 || place_pos.y >= WORLD_HEIGHT as i32 {
                return;
            }

            let target = self
                .world
                .get_block_at(place_pos.x, place_pos.y, place_pos.z);
            if target.is_solid() {
                return;
            }

            if let Some((chunk_x, chunk_z)) = self.world.set_block_at(
                place_pos.x,
                place_pos.y,
                place_pos.z,
                self.selected_block,
            ) {
                self.invalidate_chunk_and_neighbors(chunk_x, chunk_z, place_pos);
            }
        }
    }

    fn invalidate_chunk_and_neighbors(&mut self, chunk_x: i32, chunk_z: i32, world_pos: IVec3) {
        if let Some(renderer) = &mut self.renderer {
            renderer.invalidate_chunk(chunk_x, chunk_z);

            let size = CHUNK_SIZE as i32;
            let local_x = world_pos.x.rem_euclid(size) as usize;
            let local_z = world_pos.z.rem_euclid(size) as usize;

            if local_x == 0 {
                renderer.invalidate_chunk(chunk_x - 1, chunk_z);
            }
            if local_x == CHUNK_SIZE - 1 {
                renderer.invalidate_chunk(chunk_x + 1, chunk_z);
            }
            if local_z == 0 {
                renderer.invalidate_chunk(chunk_x, chunk_z - 1);
            }
            if local_z == CHUNK_SIZE - 1 {
                renderer.invalidate_chunk(chunk_x, chunk_z + 1);
            }
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;

        if self.screen != AppScreen::Playing {
            // Ensure mouse deltas don't accumulate while in menus
            self.input.reset_mouse_delta();
            return;
        }

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
        // Vertical movement: Space to ascend, Shift (either) to descend
        if self.input.is_up() {
            self.camera.position.y += speed;
        }
        if self.input.is_down() {
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
            renderer.update_chunks(&self.world, self.camera.position, render_distance);
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

            // Update camera aspect ratio
            let size = window.inner_size();
            self.camera
                .update_aspect(size.width as f32 / size.height as f32);

            // Create renderer
            let renderer = pollster::block_on(Renderer::new(
                window.clone(),
                self.settings.graphics.vsync,
            ))
            .unwrap();
            // Create egui GUI (egui_wgpu expects its own wgpu types)
            // Arc<Window> -> &Window
            let gui = Gui::new(window.as_ref(), &renderer.device, &renderer.config);

            self.renderer = Some(renderer);
            self.gui = Some(gui);
            self.window = Some(window);
            self.set_cursor_capture(false);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut egui_consumed = false;
        if let (Some(gui), Some(window_arc)) = (self.gui.as_mut(), self.window.as_ref()) {
            if window_arc.id() == window_id {
                let event_response = gui.state.on_window_event(window_arc.as_ref(), &event);
                if event_response.repaint {
                    window_arc.request_redraw();
                }
                egui_consumed = event_response.consumed;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                // Persist settings and world metadata before exiting
                let _ = self.settings.save();
                self.world.save_meta();
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size);
                    self.camera.update_aspect(
                        physical_size.width as f32 / physical_size.height as f32,
                    );
                    // TODO: resize egui renderer textures if needed
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
                if egui_consumed {
                    return;
                }
                match state {
                    ElementState::Pressed => {
                        self.input.key_pressed(key);
                        if key == KeyCode::Escape {
                            // Toggle pause when playing
                            if self.screen == AppScreen::Playing {
                                self.screen_prev = Some(self.screen);
                                self.screen = AppScreen::Paused;
                                self.set_cursor_capture(false);
                            } else if self.screen == AppScreen::Paused {
                                self.screen = AppScreen::Playing;
                                self.set_cursor_capture(true);
                            } else if self.screen == AppScreen::MainMenu {
                                event_loop.exit();
                            }
                        }
                    }
                    ElementState::Released => {
                        self.input.key_released(key);
                    }
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => {
                if egui_consumed {
                    return;
                }

                if self.screen != AppScreen::Playing {
                    return;
                }

                if !self.input.is_mouse_captured() {
                    self.set_cursor_capture(true);
                    return;
                }

                match button {
                    MouseButton::Left => self.break_block(),
                    MouseButton::Right => self.place_block(),
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                self.update();

                if self.renderer.is_some() && self.gui.is_some() && self.window.is_some() {
                    let window_arc = self.window.as_ref().cloned().unwrap();
                    let window_ref = window_arc.as_ref();
                    let renderer = self.renderer.as_mut().unwrap();
                    let gui = self.gui.as_mut().unwrap();

                    // Take raw input from winit
                    let raw_input = gui.state.take_egui_input(window_ref);

                    // Run egui::Context::run to produce layouts and paint jobs in a single
                    // closure. This ensures `available_rect()` and other layout helpers
                    // are only used while the ctx is running.
                    let run_output = gui.egui_ctx.run(raw_input, |ctx| {
                        // Draw a top bar with FPS if enabled
                        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Minecraft Clone - Rust");
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if self.settings.show_fps {
                                        ui.label(format!("FPS: {:.0}", 1.0 / self.delta_time.as_secs_f32()));
                                    }
                                });
                            });
                        });

                        // Draw main menu / worlds / settings UI depending on app screen
                        let mut settings_open = false;

                        // Allow UI to request quitting the app
                        let mut request_quit = false;

                        match self.screen {
                            AppScreen::MainMenu => {
                                // Simple main menu with navigation
                                egui::CentralPanel::default().show(ctx, |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.heading("Minecraft Clone - Rust");
                                        ui.add_space(8.0);
                                        if ui.button("Singleplayer").clicked() {
                                            self.screen = AppScreen::Worlds;
                                        }
                                        if ui.button("Multiplayer").clicked() {
                                            // no-op for now
                                        }
                                        if ui.button("Options").clicked() {
                                            self.screen_prev = Some(self.screen);
                                            self.screen = AppScreen::Settings;
                                        }
                                        if ui.button("Quit").clicked() {
                                            request_quit = true;
                                        }
                                    });
                                });
                            }
                            AppScreen::Worlds => {
                                egui::Window::new("Worlds")
                                    .resizable(true)
                                    .vscroll(true)
                                    .show(ctx, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("Available worlds:");
                                            if ui.button("Refresh").clicked() {}
                                        });

                                        // Read worlds from disk (simple alphabetical list)
                                        let mut worlds: Vec<std::path::PathBuf> = Vec::new();
                                        if let Ok(read) = std::fs::read_dir(&self.worlds_dir) {
                                            for entry in read.flatten() {
                                                let path = entry.path();
                                                if path.is_dir() { worlds.push(path); }
                                            }
                                        }
                                        worlds.sort_by_key(|p| p.file_name().map(|s| s.to_owned()));

                                        for path in worlds.iter() {
                                            let name = path.file_name().unwrap_or_default().to_string_lossy();
                                            ui.horizontal(|ui| {
                                                ui.label(name.as_ref());
                                                if ui.button("Play").clicked() {
                                                    // Load the selected world and start playing
                                                    let p = path.clone();
                                                    self.world = World::new(Some(p.clone()), None);
                                                    self.screen = AppScreen::Playing;
                                                    self.input.set_mouse_captured(true);
                                                    App::apply_cursor_capture(window_ref, true);
                                                }
                                                if ui.button("Delete").clicked() {
                                                    self.confirm_delete = Some(path.clone());
                                                }
                                            });
                                        }

                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label("Create new world:");
                                            ui.text_edit_singleline(&mut self.new_world_name);
                                            if ui.button("Create").clicked() {
                                                if !self.new_world_name.trim().is_empty() {
                                                    let mut new_path = self.worlds_dir.clone();
                                                    new_path.push(self.new_world_name.trim());
                                                    let _ = std::fs::create_dir_all(&new_path);
                                                    // Initialize world metadata by constructing World
                                                    let _ = World::new(Some(new_path.clone()), None);
                                                    self.new_world_name.clear();
                                                }
                                            }
                                            if ui.button("Back").clicked() {
                                                self.screen = AppScreen::MainMenu;
                                            }
                                        });
                                    });

                                // Show confirmation modal if deletion was requested
                                if self.confirm_delete.is_some() {
                                    // Clone the path so we don't keep a borrow on self while
                                    // mutating it inside the UI closures.
                                    let pending = self.confirm_delete.clone().unwrap();
                                    let pending_name = pending.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    egui::Window::new("Confirm Delete")
                                        .collapsible(false)
                                        .resizable(false)
                                        .show(ctx, |ui| {
                                            ui.label(format!("Delete world '{}'? This cannot be undone.", pending_name));
                                            ui.horizontal(|ui| {
                                                if ui.button("Cancel").clicked() {
                                                    self.confirm_delete = None;
                                                }
                                                if ui.button("Delete").clicked() {
                                                    if let Err(e) = std::fs::remove_dir_all(&pending) {
                                                        log::error!("Failed to delete world: {:?}", e);
                                                    }
                                                    self.confirm_delete = None;
                                                }
                                            });
                                        });
                                }
                            }
                            AppScreen::Settings => {
                                let mut open = true;
                                gui.draw_settings_window(ctx, &mut self.settings, &mut open);
                                if !open {
                                    // Return to previous screen
                                    self.screen = self.screen_prev.unwrap_or(AppScreen::MainMenu);
                                    self.screen_prev = None;
                                }
                            }
                            AppScreen::Playing => {
                                // Nothing to draw here: game UI (top bar) is rendered elsewhere
                            }
                            AppScreen::Paused => {
                                egui::Window::new("Paused").show(ctx, |ui| {
                                    if ui.button("Continue").clicked() {
                                        self.screen = AppScreen::Playing;
                                        self.input.set_mouse_captured(true);
                                        App::apply_cursor_capture(window_ref, true);
                                    }
                                    if ui.button("Options").clicked() {
                                        self.screen_prev = Some(self.screen);
                                        self.screen = AppScreen::Settings;
                                    }
                                    if ui.button("Save & Quit").clicked() {
                                        // Save world metadata and go back to main menu
                                        self.world.save_meta();
                                        let _ = self.settings.save();
                                        self.screen = AppScreen::MainMenu;
                                        self.input.set_mouse_captured(false);
                                        App::apply_cursor_capture(window_ref, false);
                                    }
                                });
                            }
                        }

                        // Draw small settings toggle if requested
                        if settings_open {
                            gui.draw_settings_window(ctx, &mut self.settings, &mut settings_open);
                        }

                        // If UI requested quit, exit the event loop after painting
                        if request_quit {
                            event_loop.exit();
                            return;
                        }
                    });

                    let egui::FullOutput {
                        platform_output,
                        textures_delta,
                        shapes,
                        pixels_per_point,
                        ..
                    } = run_output;

                    gui.state.handle_platform_output(window_ref, platform_output);

                    self.camera.fov = self.settings.graphics.fov;
                    renderer.set_vsync(self.settings.graphics.vsync);

                    let paint_jobs = gui.egui_ctx.tessellate(shapes, pixels_per_point);

                    // Acquire frame
                    let output_frame = match renderer.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            match e {
                                wgpu::SurfaceError::Lost => { renderer.resize(window_ref.inner_size()); }
                                wgpu::SurfaceError::OutOfMemory => event_loop.exit(),
                                _ => eprintln!("Failed to acquire frame: {:?}", e),
                            }
                            return;
                        }
                    };

                    let view = output_frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                    // Draw 3D scene using its own command encoder and submit it. This avoids
                    // borrow/lifetime conflicts when egui's renderer needs to use the encoder
                    // for its own update_buffers/render operations.
                    let mut scene_encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("scene encoder") });
                    if matches!(self.screen, AppScreen::Playing | AppScreen::Paused) {
                        renderer.draw_scene(&mut scene_encoder, &view);
                    } else {
                        {
                            let _ = scene_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("scene clear pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.53, g: 0.81, b: 0.92, a: 1.0 }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                    view: renderer.depth_view(),
                                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                                    stencil_ops: None,
                                }),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });
                        }
                    }
                    renderer.queue.submit(Some(scene_encoder.finish()));

                    // Prepare egui render: update buffers and textures using a separate encoder
                    let mut egui_encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("egui encoder") });

                    let screen_descriptor = egui_wgpu::ScreenDescriptor {
                        size_in_pixels: [renderer.config.width, renderer.config.height],
                        pixels_per_point,
                    };

                    gui.renderer.update_buffers(&renderer.device, &renderer.queue, &mut egui_encoder, &paint_jobs, &screen_descriptor);
                    for (id, image_delta) in &textures_delta.set {
                        gui.renderer.update_texture(&renderer.device, &renderer.queue, *id, image_delta);
                    }

                    // (UI was already built inside `egui::Context::run` earlier)

                    // Create a render pass to draw egui on top
                    {
                        let mut rpass = egui_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("egui pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                            })],
                            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                view: renderer.depth_view(),
                                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store }),
                                stencil_ops: None,
                            }),
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });

                        // Use `forget_lifetime` to drop the encoder lifetime on the render pass
                        // and obtain a `'static` render pass as required by egui_wgpu.
                        let mut rpass_static = rpass.forget_lifetime();
                        gui.renderer.render(&mut rpass_static, &paint_jobs, &screen_descriptor);
                    }

                    renderer.queue.submit(Some(egui_encoder.finish()));
                    for id in &textures_delta.free {
                        gui.renderer.free_texture(id);
                    }
                    output_frame.present();
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
    // Prepare worlds directory
    use std::fs;
    use std::path::PathBuf;

    let mut worlds_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    worlds_dir.push("minecraft-clone-rust");
    worlds_dir.push("worlds");
    let _ = fs::create_dir_all(&worlds_dir);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(worlds_dir.clone());
    event_loop.run_app(&mut app).unwrap();
}
