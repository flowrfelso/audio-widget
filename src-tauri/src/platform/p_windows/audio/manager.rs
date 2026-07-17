use anyhow::Result;

use crate::platform::p_windows::com::ComGuard;
use ::windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolumeCallback;

use super::{
    callback::{VolumeCallback, VolumeChangedHandler},
    device::default_render_device,
    endpoint::EndpointVolume,
};

pub struct AudioManager {
    _com: ComGuard,
    endpoint: EndpointVolume,
    callback: Option<IAudioEndpointVolumeCallback>,
}

unsafe impl Send for AudioManager {}
unsafe impl Sync for AudioManager {}

impl AudioManager {
    pub fn new() -> anyhow::Result<Self> {
        let com = ComGuard::new()?;

        let device = default_render_device()?;

        let endpoint = EndpointVolume::new(&device)?;

        Ok(Self {
            _com: com,
            endpoint,
            callback: None,
        })
    }

    // #[inline]
    // pub fn volume(&self) -> Result<f32> {
    //     self.endpoint.volume()
    // }

    #[inline]
    pub fn volume_percent(&self) -> Result<u8> {
        Ok((self.endpoint.volume()? * 100.0).round() as u8)
    }

    // #[inline]
    // pub fn set_volume(&self, volume: f32) -> Result<()> {
    //     self.endpoint.set_volume(volume)
    // }

    #[inline]
    pub fn set_volume_percent(&self, volume: u8) -> Result<()> {
        let volume = volume.min(100) as f32 / 100.0;

        self.endpoint.set_volume(volume)
    }

    #[inline]
    pub fn muted(&self) -> Result<bool> {
        self.endpoint.muted()
    }

    // #[inline]
    // pub fn mute(&self, value: bool) -> Result<()> {
    //     self.endpoint.set_muted(value)
    // }

    #[inline]
    pub fn toggle_mute(&self) -> Result<()> {
        let muted = self.endpoint.muted()?;

        self.endpoint.set_muted(!muted)
    }

    #[inline]
    pub fn volume_up(&self, step: u8) -> Result<()> {
        let current = self.volume_percent()?;

        let next = current.saturating_add(step).min(100);

        self.set_volume_percent(next)
    }

    #[inline]
    pub fn volume_down(&self, step: u8) -> Result<()> {
        let current = self.volume_percent()?;

        let next = current.saturating_sub(step);

        self.set_volume_percent(next)
    }

    pub fn subscribe(&mut self, handler: VolumeChangedHandler) -> anyhow::Result<()> {
        let callback = VolumeCallback::new(handler);

        let callback: IAudioEndpointVolumeCallback = callback.into();

        unsafe {
            self.endpoint.raw().RegisterControlChangeNotify(&callback)?;
        }

        self.callback = Some(callback);

        Ok(())
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            unsafe {
                let _ = self.endpoint.raw().UnregisterControlChangeNotify(callback);
            }
        }
    }
}
