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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextureQuality {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Ultra,
}

impl TextureQuality {
    pub fn tile_size(self) -> u32 {
        match self {
            TextureQuality::VeryLow => 16,
            TextureQuality::Low => 32,
            TextureQuality::Medium => 64,
            TextureQuality::High => 128,
            TextureQuality::VeryHigh => 256,
            TextureQuality::Ultra => 512,
        }
    }
}

impl fmt::Display for TextureQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TextureQuality::VeryLow => "Very Low (16x)",
            TextureQuality::Low => "Low (32x)",
            TextureQuality::Medium => "Medium (64x)",
            TextureQuality::High => "High (128x)",
            TextureQuality::VeryHigh => "Very High (256x)",
            TextureQuality::Ultra => "Ultra (512x)",
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
    pub texture_quality: TextureQuality,
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
            texture_quality: TextureQuality::Medium,
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
                self.texture_quality = TextureQuality::Low;
            }
            QualityPreset::Medium => {
                self.render_distance = 8;
                self.shadows = true;
                self.antialiasing = false;
                self.texture_quality = TextureQuality::Medium;
            }
            QualityPreset::High => {
                self.render_distance = 12;
                self.shadows = true;
                self.antialiasing = true;
                self.texture_quality = TextureQuality::High;
            }
            QualityPreset::Ultra => {
                self.render_distance = 16;
                self.shadows = true;
                self.antialiasing = true;
                self.texture_quality = TextureQuality::Ultra;
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
#[serde(default)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub mouse_sensitivity: f32,
    pub player_name: String,
    pub show_fps: bool,
    #[serde(default = "GameSettings::default_autosave_interval_secs")]
    pub autosave_interval_secs: u32,
    pub fps_cap_playing: u32,
    pub fps_cap_menu: u32,
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
            autosave_interval_secs: Self::default_autosave_interval_secs(),
            fps_cap_playing: 0,
            fps_cap_menu: 60,
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

    fn default_autosave_interval_secs() -> u32 {
        10
    }
}

// Add dirs dependency requirement
