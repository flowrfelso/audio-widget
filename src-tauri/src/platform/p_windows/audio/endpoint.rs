use anyhow::Result;

// use windows::Win32::{
//     Media::Audio::{Endpoints::IAudioEndpointVolume, IMMDevice},
//     System::Com::CLSCTX_ALL,
// };

use windows::Win32::{Media::Audio::Endpoints::*, Media::Audio::*, System::Com::*};

pub struct EndpointVolume {
    inner: IAudioEndpointVolume,
}

impl EndpointVolume {
    pub fn new(device: &IMMDevice) -> Result<Self> {
        unsafe {
            let endpoint = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)?;

            Ok(Self { inner: endpoint })
        }
    }

    #[inline]
    pub fn raw(&self) -> &IAudioEndpointVolume {
        &self.inner
    }

    #[inline]
    pub fn volume(&self) -> Result<f32> {
        unsafe { Ok(self.inner.GetMasterVolumeLevelScalar()?) }
    }

    // #[inline]
    // pub fn volume_percent(&self) -> Result<u8> {
    //     Ok((self.volume()? * 100.0).round() as u8)
    // }

    #[inline]
    pub fn set_volume(&self, value: f32) -> Result<()> {
        unsafe {
            self.inner
                .SetMasterVolumeLevelScalar(value.clamp(0.0, 1.0), std::ptr::null())?;
        }

        Ok(())
    }

    // #[inline]
    // pub fn set_volume_percent(&self, value: u8) -> Result<()> {
    //     self.set_volume(value.min(100) as f32 / 100.0)
    // }

    #[inline]
    pub fn muted(&self) -> Result<bool> {
        unsafe { Ok(self.inner.GetMute()?.as_bool()) }
    }

    #[inline]
    pub fn set_muted(&self, muted: bool) -> Result<()> {
        unsafe {
            self.inner.SetMute(muted, std::ptr::null())?;
        }

        Ok(())
    }

    // #[inline]
    // pub fn step_up(&self) -> Result<()> {
    //     unsafe {
    //         self.inner.VolumeStepUp(std::ptr::null())?;
    //     }

    //     Ok(())
    // }

    // #[inline]
    // pub fn step_down(&self) -> Result<()> {
    //     unsafe {
    //         self.inner.VolumeStepDown(std::ptr::null())?;
    //     }

    //     Ok(())
    // }
}
