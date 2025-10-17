use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityPreset {
    Low,
    Medium,
    High,
    Ultra,
    Custom,
}

impl fmt::Display for QualityPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            QualityPreset::Low => "Low",
            QualityPreset::Medium => "Medium",
            QualityPreset::High => "High",
            QualityPreset::Ultra => "Ultra",
            QualityPreset::Custom => "Custom",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub quality_preset: QualityPreset,
    pub render_distance: u32,
    pub vsync: bool,
    pub fov: f32,
    pub shadows: bool,
    pub antialiasing: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        let mut settings = Self {
            quality_preset: QualityPreset::Medium,
            render_distance: 8,
            vsync: true,
            fov: 75.0,
            shadows: true,
            antialiasing: true,
        };
        settings.apply_preset(QualityPreset::Medium);
        settings
    }
}

impl GraphicsSettings {
    pub fn apply_preset(&mut self, preset: QualityPreset) {
        self.quality_preset = preset;
        match preset {
            QualityPreset::Low => {
                self.render_distance = 4;
                self.shadows = false;
                self.antialiasing = false;
            }
            QualityPreset::Medium => {
                self.render_distance = 8;
                self.shadows = true;
                self.antialiasing = false;
            }
            QualityPreset::High => {
                self.render_distance = 12;
                self.shadows = true;
                self.antialiasing = true;
            }
            QualityPreset::Ultra => {
                self.render_distance = 16;
                self.shadows = true;
                self.antialiasing = true;
            }
            QualityPreset::Custom => {}
        }
    }

    pub fn mark_custom(&mut self) {
        if self.quality_preset != QualityPreset::Custom {
            self.quality_preset = QualityPreset::Custom;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub mouse_sensitivity: f32,
    pub player_name: String,
    pub show_fps: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings::default(),
            // Slightly higher default sensitivity for more responsive camera
            mouse_sensitivity: 0.006,
            player_name: String::from("Player"),
            // Do not show FPS overlay by default
            show_fps: false,
        }
    }
}

impl GameSettings {
    pub fn load() -> Self {
        let path = Self::get_settings_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::get_settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }

    fn get_settings_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("minecraft-clone-rust");
        path.push("settings.json");
        path
    }
}

// Add dirs dependency requirement
