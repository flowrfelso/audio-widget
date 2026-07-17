use std::{fs, path::PathBuf};

use anyhow::Result;

use super::models::Settings;

pub struct SettingsRepository {
    path: PathBuf,
}

impl SettingsRepository {
    pub fn initialize(&self) -> Result<()> {
        let _ = self.load()?;
        Ok(())
    }

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
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(settings)?;

        std::fs::write(&self.path, json)?;

        Ok(())
    }

    pub fn update<F>(&self, f: F) -> Result<Settings>
    where
        F: FnOnce(&mut Settings),
    {
        let mut settings = self.load()?;

        f(&mut settings);

        self.save(&settings)?;

        Ok(settings)
    }
}
