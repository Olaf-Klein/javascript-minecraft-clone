mod input;
mod renderer;
mod settings;
mod ui;
mod world;

use glam::{IVec3, Vec3};
use input::InputState;
use renderer::{Camera, Renderer};
use settings::GameSettings;
use std::sync::Arc;
use std::time::{Duration, Instant};
use ui::Gui;
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

const HOTBAR_BLOCKS: &[(BlockType, &str)] = &[
    (BlockType::GrassBlock, "Grass"),
    (BlockType::Dirt, "Dirt"),
    (BlockType::Stone, "Stone"),
    (BlockType::Cobblestone, "Cobble"),
    (BlockType::OakPlanks, "Planks"),
    (BlockType::Glass, "Glass"),
    (BlockType::Sand, "Sand"),
    (BlockType::Water, "Water"),
];

const PLAYER_RADIUS: f32 = 0.4;
const PLAYER_EYE_HEIGHT: f32 = 1.62;
const PLAYER_HEADROOM: f32 = 0.2;
const COLLISION_EPSILON: f32 = 0.001;
const CROSSHAIR_GAP: f32 = 5.0;
const CROSSHAIR_ARM: f32 = 12.0;

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
    selected_hotbar: usize,
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
            selected_hotbar: 0,
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

            let block = self.world.get_block_at(hit.hit.x, hit.hit.y, hit.hit.z);
            if block.hardness() < 0.0 {
                return;
            }

            if let Some((chunk_x, chunk_z)) =
                self.world
                    .set_block_at(hit.hit.x, hit.hit.y, hit.hit.z, BlockType::Air)
            {
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

            let block_to_place = self.current_block_type();
            if let Some((chunk_x, chunk_z)) =
                self.world
                    .set_block_at(place_pos.x, place_pos.y, place_pos.z, block_to_place)
            {
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

    fn current_block_type(&self) -> BlockType {
        HOTBAR_BLOCKS
            .get(self.selected_hotbar)
            .map(|(block, _)| *block)
            .unwrap_or(BlockType::GrassBlock)
    }

    fn current_block_label(&self) -> Option<&'static str> {
        HOTBAR_BLOCKS
            .get(self.selected_hotbar)
            .map(|(_, label)| *label)
    }

    fn handle_hotbar_key(&mut self, key: KeyCode) -> bool {
        let target_index = match key {
            KeyCode::Digit1 => Some(0),
            KeyCode::Digit2 => Some(1),
            KeyCode::Digit3 => Some(2),
            KeyCode::Digit4 => Some(3),
            KeyCode::Digit5 => Some(4),
            KeyCode::Digit6 => Some(5),
            KeyCode::Digit7 => Some(6),
            KeyCode::Digit8 => Some(7),
            _ => None,
        };

        if let Some(index) = target_index {
            self.select_hotbar(index);
            true
        } else {
            false
        }
    }

    fn select_hotbar(&mut self, index: usize) {
        if index < HOTBAR_BLOCKS.len() {
            self.selected_hotbar = index;
        }
    }

    fn cycle_hotbar(&mut self, offset: i32) {
        let len = HOTBAR_BLOCKS.len() as i32;
        if len == 0 {
            return;
        }
        let current = self.selected_hotbar as i32;
        let mut next = (current + offset) % len;
        if next < 0 {
            next += len;
        }
        self.selected_hotbar = next as usize;
    }

    fn adjust_hotbar_from_scroll(&mut self, delta: &MouseScrollDelta) {
        let offset = match delta {
            MouseScrollDelta::LineDelta(_, y) => y.signum() as i32,
            MouseScrollDelta::PixelDelta(pos) => pos.y.signum() as i32,
        };
        if offset != 0 {
            self.cycle_hotbar(-offset);
        }
    }

    fn move_with_collisions(&mut self, position: Vec3, motion: Vec3) -> Vec3 {
        let mut pos = position;
        if motion.x.abs() > f32::EPSILON {
            pos = self.move_axis_x(pos, motion.x);
        }
        if motion.y.abs() > f32::EPSILON {
            pos = self.move_axis_y(pos, motion.y);
        }
        if motion.z.abs() > f32::EPSILON {
            pos = self.move_axis_z(pos, motion.z);
        }
        pos
    }

    fn move_axis_x(&mut self, position: Vec3, dx: f32) -> Vec3 {
        let mut pos = position;
        let mut new_x = pos.x + dx;

        let min_y = (pos.y - PLAYER_EYE_HEIGHT).floor() as i32;
        let max_y = (pos.y + PLAYER_HEADROOM).floor() as i32;
        let min_z = (pos.z - PLAYER_RADIUS).floor() as i32;
        let max_z = (pos.z + PLAYER_RADIUS).floor() as i32;

        if dx > 0.0 {
            let max_x = (new_x + PLAYER_RADIUS).floor() as i32;
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if self.world.get_block_at(max_x, y, z).is_solid() {
                        new_x = max_x as f32 - PLAYER_RADIUS - COLLISION_EPSILON;
                        pos.x = new_x;
                        return pos;
                    }
                }
            }
        } else {
            let min_x = (new_x - PLAYER_RADIUS).floor() as i32;
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if self.world.get_block_at(min_x, y, z).is_solid() {
                        new_x = min_x as f32 + 1.0 + PLAYER_RADIUS + COLLISION_EPSILON;
                        pos.x = new_x;
                        return pos;
                    }
                }
            }
        }

        pos.x = new_x;
        pos
    }

    fn move_axis_y(&mut self, position: Vec3, dy: f32) -> Vec3 {
        let mut pos = position;
        let mut new_y = pos.y + dy;

        let min_x = (pos.x - PLAYER_RADIUS).floor() as i32;
        let max_x = (pos.x + PLAYER_RADIUS).floor() as i32;
        let min_z = (pos.z - PLAYER_RADIUS).floor() as i32;
        let max_z = (pos.z + PLAYER_RADIUS).floor() as i32;

        if dy > 0.0 {
            let max_y = (new_y + PLAYER_HEADROOM).floor() as i32;
            for x in min_x..=max_x {
                for z in min_z..=max_z {
                    if self.world.get_block_at(x, max_y, z).is_solid() {
                        new_y = max_y as f32 - PLAYER_HEADROOM - COLLISION_EPSILON;
                        pos.y = new_y;
                        return pos;
                    }
                }
            }
        } else {
            let min_y = (new_y - PLAYER_EYE_HEIGHT).floor() as i32;
            for x in min_x..=max_x {
                for z in min_z..=max_z {
                    if self.world.get_block_at(x, min_y, z).is_solid() {
                        new_y = min_y as f32 + 1.0 + PLAYER_EYE_HEIGHT + COLLISION_EPSILON;
                        pos.y = new_y;
                        return pos;
                    }
                }
            }
        }

        pos.y = new_y;
        pos
    }

    fn move_axis_z(&mut self, position: Vec3, dz: f32) -> Vec3 {
        let mut pos = position;
        let mut new_z = pos.z + dz;

        let min_y = (pos.y - PLAYER_EYE_HEIGHT).floor() as i32;
        let max_y = (pos.y + PLAYER_HEADROOM).floor() as i32;
        let min_x = (pos.x - PLAYER_RADIUS).floor() as i32;
        let max_x = (pos.x + PLAYER_RADIUS).floor() as i32;

        if dz > 0.0 {
            let max_z = (new_z + PLAYER_RADIUS).floor() as i32;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if self.world.get_block_at(x, y, max_z).is_solid() {
                        new_z = max_z as f32 - PLAYER_RADIUS - COLLISION_EPSILON;
                        pos.z = new_z;
                        return pos;
                    }
                }
            }
        } else {
            let min_z = (new_z - PLAYER_RADIUS).floor() as i32;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if self.world.get_block_at(x, y, min_z).is_solid() {
                        new_z = min_z as f32 + 1.0 + PLAYER_RADIUS + COLLISION_EPSILON;
                        pos.z = new_z;
                        return pos;
                    }
                }
            }
        }

        pos.z = new_z;
        pos
    }

    fn draw_hotbar_overlay(
        ctx: &egui::Context,
        selected_hotbar: usize,
        current_label: Option<&'static str>,
    ) -> Option<usize> {
        if HOTBAR_BLOCKS.is_empty() {
            return None;
        }

        let slot_size = 48.0;
        let spacing = 6.0;
        let mut selection: Option<usize> = None;
        let mut active_label = current_label;

        egui::TopBottomPanel::bottom("hotbar_panel")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 170))
                    .inner_margin(egui::Margin::symmetric(20.0, 14.0)),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if let Some(label) = active_label {
                        ui.label(
                            egui::RichText::new(label)
                                .color(egui::Color32::from_rgb(245, 245, 245))
                                .size(18.0),
                        );
                        ui.add_space(6.0);
                    }

                    ui.horizontal_centered(|ui| {
                        ui.spacing_mut().item_spacing.x = spacing;
                        for (index, (_, label)) in HOTBAR_BLOCKS.iter().enumerate() {
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(slot_size, slot_size),
                                egui::Sense::click(),
                            );
                            if response.clicked() {
                                selection = Some(index);
                                active_label = Some(*label);
                            }

                            let is_selected = selection.unwrap_or(selected_hotbar) == index;
                            let painter = ui.painter_at(rect);
                            let base_fill = egui::Color32::from_rgb(36, 36, 36);
                            let base_stroke =
                                egui::Stroke::new(1.5, egui::Color32::from_rgb(70, 70, 70));
                            painter.rect(rect, egui::Rounding::same(8.0), base_fill, base_stroke);

                            let inner = rect.shrink(6.0);
                            let (inner_fill, inner_stroke) = if is_selected {
                                (
                                    egui::Color32::from_rgb(180, 170, 110),
                                    egui::Stroke::new(2.5, egui::Color32::from_rgb(255, 220, 120)),
                                )
                            } else {
                                (
                                    egui::Color32::from_rgb(58, 58, 58),
                                    egui::Stroke::new(1.0, egui::Color32::from_rgb(15, 15, 15)),
                                )
                            };
                            painter.rect(
                                inner,
                                egui::Rounding::same(6.0),
                                inner_fill,
                                inner_stroke,
                            );

                            let text_color = if is_selected {
                                egui::Color32::from_rgb(32, 32, 32)
                            } else {
                                egui::Color32::from_rgb(220, 220, 220)
                            };
                            painter.text(
                                inner.center(),
                                egui::Align2::CENTER_CENTER,
                                *label,
                                egui::FontId::proportional(15.0),
                                text_color,
                            );

                            painter.text(
                                rect.left_bottom() + egui::vec2(6.0, -4.0),
                                egui::Align2::LEFT_BOTTOM,
                                format!("{}", index + 1),
                                egui::FontId::proportional(13.0),
                                egui::Color32::from_rgb(200, 200, 200),
                            );
                        }
                    });
                });
            });

        selection
    }

    fn draw_crosshair(ctx: &egui::Context) {
        let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("crosshair"));
        let painter = ctx.layer_painter(layer_id);
        let center = ctx.input(|i| i.screen_rect().center());

        let segments = [
            (
                egui::pos2(center.x - CROSSHAIR_ARM, center.y),
                egui::pos2(center.x - CROSSHAIR_GAP, center.y),
            ),
            (
                egui::pos2(center.x + CROSSHAIR_GAP, center.y),
                egui::pos2(center.x + CROSSHAIR_ARM, center.y),
            ),
            (
                egui::pos2(center.x, center.y - CROSSHAIR_ARM),
                egui::pos2(center.x, center.y - CROSSHAIR_GAP),
            ),
            (
                egui::pos2(center.x, center.y + CROSSHAIR_GAP),
                egui::pos2(center.x, center.y + CROSSHAIR_ARM),
            ),
        ];

        let shadow = egui::Stroke::new(4.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 120));
        let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(250, 250, 250));

        for &(a, b) in &segments {
            painter.line_segment([a, b], shadow);
        }
        for &(a, b) in &segments {
            painter.line_segment([a, b], stroke);
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

        let front = self.camera.get_front();
        let right = self.camera.get_right();
        let mut movement = Vec3::ZERO;

        if self.input.is_forward() {
            movement += front;
        }
        if self.input.is_backward() {
            movement -= front;
        }
        if self.input.is_left() {
            movement -= right;
        }
        if self.input.is_right() {
            movement += right;
        }
        if self.input.is_up() {
            movement += Vec3::Y;
        }
        if self.input.is_down() {
            movement -= Vec3::Y;
        }

        if movement.length_squared() > f32::EPSILON {
            let movement = movement.normalize_or_zero() * speed;
            self.camera.position = self.move_with_collisions(self.camera.position, movement);
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
            for z in (player_chunk_x - render_distance)..=(player_chunk_x + render_distance) {
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
                .with_title("Minecraft Clone")
                .with_inner_size(winit::dpi::LogicalSize::new(1200, 800));

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

            // Update camera aspect ratio
            let size = window.inner_size();
            self.camera
                .update_aspect(size.width as f32 / size.height as f32);

            // Create renderer
            let renderer =
                pollster::block_on(Renderer::new(window.clone(), self.settings.graphics.vsync))
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
                    self.camera
                        .update_aspect(physical_size.width as f32 / physical_size.height as f32);
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
                        let _ = self.handle_hotbar_key(key);
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
            WindowEvent::MouseWheel { delta, .. } => {
                if egui_consumed {
                    return;
                }
                if matches!(self.screen, AppScreen::Playing | AppScreen::Paused) {
                    self.adjust_hotbar_from_scroll(&delta);
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
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
                    let mut pending_hotbar_selection: Option<usize> = None;
                    let window_arc = self.window.as_ref().cloned().unwrap();
                    let window_ref = window_arc.as_ref();
                    let selected_hotbar = self.selected_hotbar;
                    let current_hotbar_label = self.current_block_label();
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
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if self.settings.show_fps {
                                            ui.label(format!(
                                                "FPS: {:.0}",
                                                1.0 / self.delta_time.as_secs_f32()
                                            ));
                                        }
                                    },
                                );
                            });
                        });

                        if matches!(self.screen, AppScreen::Playing | AppScreen::Paused) {
                            if let Some(new_selection) =
                                App::draw_hotbar_overlay(ctx, selected_hotbar, current_hotbar_label)
                            {
                                pending_hotbar_selection = Some(new_selection);
                            }
                        }
                        if matches!(self.screen, AppScreen::Playing) {
                            App::draw_crosshair(ctx);
                        }

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
                                                if path.is_dir() {
                                                    worlds.push(path);
                                                }
                                            }
                                        }
                                        worlds.sort_by_key(|p| p.file_name().map(|s| s.to_owned()));

                                        for path in worlds.iter() {
                                            let name = path
                                                .file_name()
                                                .unwrap_or_default()
                                                .to_string_lossy();
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
                                                    let _ =
                                                        World::new(Some(new_path.clone()), None);
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
                                    let pending_name = pending
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string();
                                    egui::Window::new("Confirm Delete")
                                        .collapsible(false)
                                        .resizable(false)
                                        .show(ctx, |ui| {
                                            ui.label(format!(
                                                "Delete world '{}'? This cannot be undone.",
                                                pending_name
                                            ));
                                            ui.horizontal(|ui| {
                                                if ui.button("Cancel").clicked() {
                                                    self.confirm_delete = None;
                                                }
                                                if ui.button("Delete").clicked() {
                                                    if let Err(e) =
                                                        std::fs::remove_dir_all(&pending)
                                                    {
                                                        log::error!(
                                                            "Failed to delete world: {:?}",
                                                            e
                                                        );
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

                    gui.state
                        .handle_platform_output(window_ref, platform_output);

                    let renderer = self.renderer.as_mut().unwrap();
                    self.camera.fov = self.settings.graphics.fov;
                    renderer.set_vsync(self.settings.graphics.vsync);

                    let paint_jobs = gui.egui_ctx.tessellate(shapes, pixels_per_point);

                    // Acquire frame
                    let output_frame = match renderer.surface.get_current_texture() {
                        Ok(frame) => frame,
                        Err(e) => {
                            match e {
                                wgpu::SurfaceError::Lost => {
                                    renderer.resize(window_ref.inner_size());
                                }
                                wgpu::SurfaceError::OutOfMemory => event_loop.exit(),
                                _ => eprintln!("Failed to acquire frame: {:?}", e),
                            }
                            return;
                        }
                    };

                    let view = output_frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    // Draw 3D scene using its own command encoder and submit it. This avoids
                    // borrow/lifetime conflicts when egui's renderer needs to use the encoder
                    // for its own update_buffers/render operations.
                    let mut scene_encoder =
                        renderer
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("scene encoder"),
                            });
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
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.53,
                                            g: 0.81,
                                            b: 0.92,
                                            a: 1.0,
                                        }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: renderer.depth_view(),
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(1.0),
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    },
                                ),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });
                        }
                    }
                    renderer.queue.submit(Some(scene_encoder.finish()));

                    // Prepare egui render: update buffers and textures using a separate encoder
                    let mut egui_encoder =
                        renderer
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("egui encoder"),
                            });

                    let screen_descriptor = egui_wgpu::ScreenDescriptor {
                        size_in_pixels: [renderer.config.width, renderer.config.height],
                        pixels_per_point,
                    };

                    gui.renderer.update_buffers(
                        &renderer.device,
                        &renderer.queue,
                        &mut egui_encoder,
                        &paint_jobs,
                        &screen_descriptor,
                    );
                    for (id, image_delta) in &textures_delta.set {
                        gui.renderer.update_texture(
                            &renderer.device,
                            &renderer.queue,
                            *id,
                            image_delta,
                        );
                    }

                    // (UI was already built inside `egui::Context::run` earlier)

                    // Create a render pass to draw egui on top
                    {
                        let mut rpass =
                            egui_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("egui pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: renderer.depth_view(),
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Load,
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    },
                                ),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                        // Use `forget_lifetime` to drop the encoder lifetime on the render pass
                        // and obtain a `'static` render pass as required by egui_wgpu.
                        let mut rpass_static = rpass.forget_lifetime();
                        gui.renderer
                            .render(&mut rpass_static, &paint_jobs, &screen_descriptor);
                    }

                    renderer.queue.submit(Some(egui_encoder.finish()));
                    for id in &textures_delta.free {
                        gui.renderer.free_texture(id);
                    }
                    output_frame.present();

                    if let Some(index) = pending_hotbar_selection {
                        self.select_hotbar(index);
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
    log::info!("Starting Minecraft Clone");
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
