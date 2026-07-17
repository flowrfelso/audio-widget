use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Wry};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SettingKey {
    AlwaysOnTop,
    LaunchAtStartup,
}

use crate::{
    services::{
        settings_service::SettingsService,
        window_service::{StartupService, WindowService},
    },
    state::tray::TrayState,
    ui::{
        settings::{models::Settings, repository::SettingsRepository},
        tray,
    },
};

#[tauri::command]
pub fn get_settings(_app: AppHandle<Wry>) -> Result<Settings, String> {
    SettingsRepository::new()
        .map_err(|e| e.to_string())?
        .load()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_setting(
    app: AppHandle<Wry>,
    key: SettingKey,
    value: bool,
) -> Result<Settings, String> {
    let repo = SettingsRepository::new().map_err(|e| e.to_string())?;

    let settings = repo
        .update(|s| match key {
            SettingKey::AlwaysOnTop => s.always_on_top = value,
            SettingKey::LaunchAtStartup => s.launch_at_startup = value,
        })
        .map_err(|e| e.to_string())?;

    SettingsService::apply(&app, &settings).map_err(|e| e.to_string())?;

    app.emit("setting-changed", (key, value))
        .map_err(|e| e.to_string())?;

    Ok(settings)
}
