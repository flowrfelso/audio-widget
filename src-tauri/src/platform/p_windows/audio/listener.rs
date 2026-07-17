use std::sync::Arc;

use tauri::{AppHandle, Emitter};

use crate::events::AudioChangedEvent;

use super::{
    callback::VolumeChangedHandler,
    manager::AudioManager,
};

pub struct AudioListener;

impl AudioListener {
    pub fn start(
        app: AppHandle,
    ) -> anyhow::Result<AudioManager> {
        let mut manager = AudioManager::new()?;

        let handler: VolumeChangedHandler = Arc::new(move |volume, muted| {
            let _ = app.emit(
                "audio://changed",
                AudioChangedEvent {
                    volume: (volume * 100.0).round() as u8,
                    muted,
                },
            );
        });

        manager.subscribe(handler)?;

        Ok(manager)
    }
}