use anyhow::Result;

use windows::Win32::{
    Media::Audio::{eConsole, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{CoCreateInstance, CLSCTX_ALL},
};

pub fn default_render_device() -> Result<IMMDevice> {
    unsafe {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        Ok(enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?)
    }
}
