use anyhow::Result;
use tauri::{AppHandle, Emitter, Wry};

use crate::{
    commands::settings::SettingKey,
    services::window_service::{StartupService, WindowService},
    ui::{settings::repository::SettingsRepository, tray::menu::TrayMenu},
};

pub struct AppManager;

impl AppManager {
    pub fn initialize(app: &AppHandle<Wry>) -> Result<()> {
        let repo = SettingsRepository::new()?;

        repo.initialize()?;

        let settings = repo.load()?;

        WindowService::apply(app, settings.always_on_top)?;
        WindowService::create(app)?;

        StartupService::apply(app, settings.launch_at_startup)?;

        Ok(())
    }

    pub fn toggle_pin(app: &AppHandle<Wry>, tray: &TrayMenu) -> Result<()> {
        let repo = SettingsRepository::new()?;

        let settings = repo.update(|s| {
            s.always_on_top = !s.always_on_top;
        })?;

        tray.pin.set_checked(settings.always_on_top)?;

        WindowService::apply(app, settings.always_on_top)?;

        app.emit(
            "setting-changed",
            (SettingKey::AlwaysOnTop, settings.always_on_top),
        )?;

        Ok(())
    }

    // pub fn toggle_launch_at_startup(app: &AppHandle<Wry>, tray: &TrayMenu) -> Result<()> {
    //     let repo = SettingsRepository::new()?;

    //     let settings = repo.update(|s| {
    //         s.launch_at_startup = !s.launch_at_startup;
    //     })?;

    //     StartupService::apply(app, settings.launch_at_startup)?;

    //     // tray.startup.set_checked(settings.launch_at_startup)?;

    //     Ok(())
    // }
}
