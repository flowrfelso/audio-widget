use tauri::{AppHandle, Manager, Wry};

use crate::{state::tray::TrayState, ui::settings::models::Settings};

pub struct TrayService;

impl TrayService {
    pub fn sync(app: &AppHandle<Wry>, settings: &Settings) -> anyhow::Result<()> {
        let tray = app.state::<TrayState>();

        tray.pin.set_checked(settings.always_on_top)?;

        Ok(())
    }
}
