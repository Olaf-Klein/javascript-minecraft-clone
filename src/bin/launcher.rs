use eframe::{egui};
use std::path::{PathBuf};
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result as AnyResult;

const LAUNCHER_CONFIG: &str = "launcher_config.json";

#[derive(Default)]
struct ModEntry {
    name: String,
    path: PathBuf,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct LauncherConfig {
    game_path: Option<PathBuf>,
    disabled_mods: Vec<String>,
    window_size: Option<(f32, f32)>,
    last_selected: Option<String>,
}

struct LauncherApp {
    game_path: Option<PathBuf>,
    mods: Vec<ModEntry>,
    selected: Option<usize>,
    error: Option<String>,
    config: LauncherConfig,
}

impl Default for LauncherApp {
    fn default() -> Self {
        let mut app = LauncherApp {
            game_path: None,
            mods: Vec::new(),
            selected: None,
            error: None,
            config: LauncherConfig::default(),
        };
        app.load_config();
        app.scan_mods_dir();
        // restore last selected if present
        if let Some(name) = &app.config.last_selected {
            for (i, m) in app.mods.iter().enumerate() {
                if &m.name == name {
                    app.selected = Some(i);
                    break;
                }
            }
        }
        app
    }
}

impl LauncherApp {
    fn scan_mods_dir(&mut self) {
        self.mods.clear();
        let mods_dir = PathBuf::from("mods");
        if let Ok(entries) = fs::read_dir(&mods_dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().and_then(|s| s.to_str()) == Some("rhai") {
                    let name = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                    let enabled = !self.config.disabled_mods.iter().any(|d| d == &name);
                    self.mods.push(ModEntry { name, path: p, enabled });
                }
            }
        }
    }

    fn launch_game(&mut self) {
        if let Some(path) = &self.game_path {
            if !path.exists() {
                self.error = Some("Game executable not found".to_string());
                return;
            }
            // Launch process detached and set working directory to exe parent
            if let Some(parent) = path.parent() {
                #[cfg(target_os = "windows")]
                {
                    // Use cmd start to detach on Windows
                    let exe = path.to_string_lossy().to_string();
                    let _ = Command::new("cmd")
                        .args(["/C", "start", "", &exe])
                        .current_dir(parent)
                        .spawn();
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = Command::new(path).current_dir(parent).spawn();
                }
            } else {
                let _ = Command::new(path).spawn();
            }
        } else {
            self.error = Some("Game path not set".to_string());
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(i) = self.selected {
            if let Some(m) = self.mods.get_mut(i) {
                m.enabled = !m.enabled;
            }
        }
    }

    fn remove_selected(&mut self) {
        if let Some(i) = self.selected {
            if let Some(m) = self.mods.get(i) {
                let _ = fs::remove_file(&m.path);
            }
            self.scan_mods_dir();
            self.selected = None;
            let _ = self.save_config();
        }
    }

    fn load_config(&mut self) {
        if let Ok(data) = fs::read_to_string(LAUNCHER_CONFIG) {
            if let Ok(cfg) = serde_json::from_str::<LauncherConfig>(&data) {
                self.config = cfg;
                self.game_path = self.config.game_path.clone();
            }
        }
    }

    fn save_config(&self) -> AnyResult<()> {
        let data = serde_json::to_string_pretty(&self.config)?;
        fs::write(LAUNCHER_CONFIG, data)?;
        Ok(())
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minecraft Clone Launcher");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Game path:");
                if ui.button("Browse").clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("Executable", &["exe", ""]).pick_file() {
                        self.game_path = Some(p.clone());
                        self.config.game_path = Some(p);
                        let _ = self.save_config();
                    }
                }
                if let Some(p) = &self.game_path {
                    ui.label(p.to_string_lossy());
                } else {
                    ui.label("<not set>");
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    self.launch_game();
                }
                if ui.button("Refresh Mods").clicked() {
                    self.scan_mods_dir();
                }
                if ui.button("Open Mods Folder").clicked() {
                    let _ = opener::open("mods");
                }
                if ui.button("Save").clicked() {
                    // save window size and last selected
                    let size = ctx.used_size();
                    self.config.window_size = Some((size.x, size.y));
                    if let Some(i) = self.selected {
                        if let Some(m) = self.mods.get(i) {
                            self.config.last_selected = Some(m.name.clone());
                        }
                    }
                    let _ = self.save_config();
                }
            });

            ui.separator();
            ui.heading("Mods");
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Collect changes to apply after UI to avoid multiple mutable borrows of self
                let mut changed_enabled: Vec<(usize, bool)> = Vec::new();
                let mut clicked_select_idx: Option<usize> = None;

                for i in 0..self.mods.len() {
                    let name = self.mods[i].name.clone();
                    let mut enabled = self.mods[i].enabled;
                    let selected_eq = self.selected == Some(i);

                    ui.horizontal(|ui| {
                        if ui.selectable_label(selected_eq, &name).clicked() {
                            clicked_select_idx = Some(i);
                        }
                        if ui.checkbox(&mut enabled, "Enabled").changed() {
                            changed_enabled.push((i, enabled));
                        }
                    });
                }

                // Apply changes after UI pass
                if let Some(i) = clicked_select_idx {
                    self.selected = Some(i);
                    if let Some(m) = self.mods.get(i) {
                        self.config.last_selected = Some(m.name.clone());
                    }
                }
                if !changed_enabled.is_empty() {
                    for (i, new_enabled) in changed_enabled {
                        if let Some(m) = self.mods.get_mut(i) {
                            m.enabled = new_enabled;
                            if new_enabled {
                                self.config.disabled_mods.retain(|d| d != &m.name);
                            } else {
                                if !self.config.disabled_mods.iter().any(|d| d == &m.name) {
                                    self.config.disabled_mods.push(m.name.clone());
                                }
                            }
                        }
                    }
                    let _ = self.save_config();
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Toggle Selected").clicked() {
                    self.toggle_selected();
                }
                if ui.button("Remove Selected").clicked() {
                    self.remove_selected();
                }
            });

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }
        });
    }
}

fn main() {
    let app = LauncherApp::default();
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Minecraft Clone Launcher", native_options, Box::new(|_cc| Ok(Box::new(app))));
}
