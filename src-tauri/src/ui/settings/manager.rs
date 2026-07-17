use std::{fs, path::PathBuf};

use anyhow::Result;

use super::models::Settings;

pub struct SettingsManager {
    path: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Result<Self> {
        let dir = dirs::config_dir().unwrap().join("AudioWidget");

        fs::create_dir_all(&dir)?;

        Ok(Self {
            path: dir.join("settings.json"),
        })
    }

    pub fn load(&self) -> Result<Settings> {
        if !self.path.exists() {
            let settings = Settings::default();

            self.save(&settings)?;

            return Ok(settings);
        }

        let json = fs::read_to_string(&self.path)?;

        Ok(serde_json::from_str(&json)?)
    }

    pub fn save(&self, settings: &Settings) -> Result<()> {
        let json = serde_json::to_string_pretty(settings)?;

        fs::write(&self.path, json)?;

        Ok(())
    }
}
