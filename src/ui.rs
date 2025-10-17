use crate::settings::{GameSettings, QualityPreset};
use egui::{self, FontDefinitions};
use egui_winit::State;
use winit::window::Window;

pub struct Gui {
    pub egui_ctx: egui::Context,
    pub state: egui_winit::State,
    pub renderer: egui_wgpu::Renderer,
}

impl Gui {
    pub fn new(
        window: &Window,
        device: &egui_wgpu::wgpu::Device,
        config: &egui_wgpu::wgpu::SurfaceConfiguration,
    ) -> Self {
        let egui_ctx = egui::Context::default();
        let fonts = FontDefinitions::default();
        egui_ctx.set_fonts(fonts);

        // Construct State with required arguments (egui_ctx, default viewport id, window,
        // native pixels per point, optional theme, optional max texture side).
        let native_pixels_per_point = Some(window.scale_factor() as f32);
        let state = State::new(
            egui_ctx.clone(),
            egui::ViewportId::default(),
            window,
            native_pixels_per_point,
            None,
            None,
        );

        // Create renderer using egui_wgpu's wgpu types to avoid cross-crate type mismatch
        let renderer = egui_wgpu::Renderer::new(
            device,
            config.format,
            Some(egui_wgpu::wgpu::TextureFormat::Depth32Float),
            1,
            true,
        );

        Self {
            egui_ctx,
            state,
            renderer,
        }
    }

    /// Draw the main in-window menu. Returns whether the user requested to open settings.
    pub fn draw_main_menu(&self, ctx: &egui::Context, settings_open: &mut bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Minecraft Clone");
                ui.add_space(8.0);
                if ui.button("Continue").clicked() {
                    // No-op: main loop will continue
                }
                if ui.button("Worlds").clicked() {
                    // Show a worlds panel - in this simple pass we toggle settings to reuse
                    *settings_open = true;
                }
                if ui.button("Settings").clicked() {
                    *settings_open = true;
                }
                if ui.button("Quit").clicked() {
                    // The application will receive a CloseRequested elsewhere (escape) — leave for now
                }
            });
        });
    }

    /// Draw settings window. The settings reference is mutated directly by the UI.
    pub fn draw_settings_window(
        &self,
        ctx: &egui::Context,
        settings: &mut GameSettings,
        open: &mut bool,
    ) {
        egui::Window::new("Settings")
            .resizable(true)
            .open(open)
            .show(ctx, |ui| {
                ui.heading("Graphics");

                let mut selected_preset = settings.graphics.quality_preset;
                egui::ComboBox::from_label("Graphics Preset")
                    .selected_text(selected_preset.to_string())
                    .show_ui(ui, |ui| {
                        for preset in [
                            QualityPreset::Low,
                            QualityPreset::Medium,
                            QualityPreset::High,
                            QualityPreset::Ultra,
                            QualityPreset::Custom,
                        ] {
                            ui.selectable_value(&mut selected_preset, preset, preset.to_string());
                        }
                    });
                if selected_preset != settings.graphics.quality_preset {
                    settings.graphics.apply_preset(selected_preset);
                }

                let render_distance_response = ui.add(
                    egui::Slider::new(&mut settings.graphics.render_distance, 2..=32)
                        .text("Render Distance (chunks)"),
                );
                if render_distance_response.changed() {
                    settings.graphics.mark_custom();
                }

                let fov_response = ui.add(
                    egui::Slider::new(&mut settings.graphics.fov, 60.0..=110.0)
                        .text("Field of View"),
                );
                if fov_response.changed() {
                    settings.graphics.mark_custom();
                }

                let vsync_response = ui.checkbox(&mut settings.graphics.vsync, "VSync");
                if vsync_response.changed() {
                    settings.graphics.mark_custom();
                }

                let shadows_response = ui.checkbox(&mut settings.graphics.shadows, "Shadows");
                if shadows_response.changed() {
                    settings.graphics.mark_custom();
                }

                let aa_response = ui.checkbox(&mut settings.graphics.antialiasing, "Antialiasing");
                if aa_response.changed() {
                    settings.graphics.mark_custom();
                }

                ui.add_space(8.0);
                ui.heading("Gameplay");
                ui.add(
                    egui::Slider::new(&mut settings.mouse_sensitivity, 0.001..=0.05)
                        .text("Mouse Sensitivity"),
                );
                if ui
                    .checkbox(&mut settings.show_fps, "Show FPS Overlay")
                    .changed()
                {
                    let _ = settings.save();
                }

                ui.add_space(8.0);
                if ui.button("Save Settings").clicked() {
                    let _ = settings.save();
                }
            });
    }
}
