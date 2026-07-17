use std::sync::Arc;

use windows::{
    core::{implement, Result},
    Win32::Media::Audio::{
        Endpoints::{IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Impl},
        AUDIO_VOLUME_NOTIFICATION_DATA,
    },
};

pub type VolumeChangedHandler = Arc<dyn Fn(f32, bool) + Send + Sync + 'static>;

#[implement(IAudioEndpointVolumeCallback)]
pub struct VolumeCallback {
    handler: VolumeChangedHandler,
}

impl VolumeCallback {
    pub fn new(handler: VolumeChangedHandler) -> Self {
        Self { handler }
    }
}

#[allow(non_snake_case)]
impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
    fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> Result<()> {
        unsafe {
            if pnotify.is_null() {
                return Ok(());
            }

            let notify = &*pnotify;

            (self.handler)(notify.fMasterVolume, notify.bMuted.as_bool());
        }

        Ok(())
    }
}
