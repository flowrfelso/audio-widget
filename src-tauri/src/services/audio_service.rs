use anyhow::Result;

use crate::platform::p_windows::audio::AudioManager;

pub struct AudioService {
    manager: AudioManager,
}

impl AudioService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            manager: AudioManager::new()?,
        })
    }

    pub fn volume(&self) -> Result<u8> {
        self.manager.volume_percent()
    }

    pub fn set_volume(&self, volume: u8) -> Result<()> {
        self.manager.set_volume_percent(volume)
    }

    pub fn volume_up(&self, step: u8) -> Result<()> {
        self.manager.volume_up(step)
    }

    pub fn volume_down(&self, step: u8) -> Result<()> {
        self.manager.volume_down(step)
    }

    pub fn muted(&self) -> Result<bool> {
        self.manager.muted()
    }

    pub fn toggle_mute(&self) -> Result<()> {
        self.manager.toggle_mute()
    }
}
