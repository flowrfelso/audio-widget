use tauri::{AppHandle, Wry};

use crate::{
    services::{
        tray_service::TrayService,
        window_service::{StartupService, WindowService},
    },
    ui::settings::models::Settings,
};

pub struct SettingsService;

impl SettingsService {
    pub fn apply(app: &AppHandle<Wry>, settings: &Settings) -> anyhow::Result<()> {
        WindowService::apply(app, settings.always_on_top)?;
        StartupService::apply(app, settings.launch_at_startup)?;
        TrayService::sync(app, settings)?;

        Ok(())
    }
}
