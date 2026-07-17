use anyhow::Result;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_autostart::ManagerExt;

pub struct WindowService;
pub struct StartupService;

impl WindowService {
    pub fn apply(app: &AppHandle<Wry>, always_on_top: bool) -> Result<()> {
        if let Some(window) = app.get_webview_window("main") {
            window.set_always_on_top(always_on_top)?;
        }

        Ok(())
    }
}

impl StartupService {
    pub fn apply(app: &AppHandle<Wry>, launch: bool) -> Result<()> {
        if launch {
            Self::enable(app)?;
        } else {
            Self::disable(app)?;
        }

        Ok(())
    }

    pub fn enable(app: &AppHandle<Wry>) -> Result<()> {
        app.autolaunch().enable()?;
        Ok(())
    }

    pub fn disable(app: &AppHandle<Wry>) -> Result<()> {
        app.autolaunch().disable()?;
        Ok(())
    }

    pub fn enabled(app: &AppHandle<Wry>) -> Result<bool> {
        Ok(app.autolaunch().is_enabled()?)
    }
}
