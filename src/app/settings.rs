use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    pub theme: ThemeSettings,
    pub scrolling: ScrollingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub background_color: String,
    pub foreground_color: String,
    pub status_bar_fg: String,
    pub status_bar_bg: String,
    pub scrollbar_track: String,
    pub scrollbar_thumb: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollingSettings {
    pub scroll_speed_lines: usize,
    pub scroll_by_page: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            background_color: "black".to_string(),
            foreground_color: "white".to_string(),
            status_bar_fg: "yellow".to_string(),
            status_bar_bg: "darkgray".to_string(),
            scrollbar_track: "darkgray".to_string(),
            scrollbar_thumb: "white".to_string(),
        }
    }
}

impl Default for ScrollingSettings {
    fn default() -> Self {
        Self {
            scroll_speed_lines: 1,
            scroll_by_page: true,
        }
    }
}

impl Settings {
    /// Get the path to the settings file
    fn settings_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        let app_dir = config_dir.join("lostower");
        fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("settings.toml"))
    }

    /// Load settings from file, or return default if file doesn't exist or can't be read
    pub fn load() -> Self {
        match Self::settings_path() {
            Ok(path) => {
                if !path.exists() {
                    let default = Settings::default();
                    let _ = default.save(); // Ignore save errors on first load
                    default
                } else {
                    match fs::read_to_string(path) {
                        Ok(content) => toml::from_str(&content).unwrap_or_default(),
                        Err(_) => Settings::default(),
                    }
                }
            }
            Err(_) => Settings::default(),
        }
    }

    /// Save settings to file
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::settings_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

