use std::sync::Mutex;

use crate::platform::p_windows::audio::AudioManager;

pub struct AudioState {
    pub manager: Mutex<AudioManager>,
}

impl std::ops::Deref for AudioState {
    type Target = Mutex<AudioManager>;

    fn deref(&self) -> &Self::Target {
        &self.manager
    }
}
