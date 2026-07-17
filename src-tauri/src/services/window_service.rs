use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindow, Wry};
use tauri_plugin_autostart::ManagerExt;

use crate::ui::window::{labels, settings};

pub struct WindowService;
pub struct StartupService;

impl WindowService {
    pub fn apply(app: &AppHandle<Wry>, always_on_top: bool) -> Result<()> {
        if let Some(window) = app.get_webview_window("main") {
            window.set_always_on_top(always_on_top)?;
        }

        Ok(())
    }

    pub fn widget(app: &AppHandle<Wry>) -> WebviewWindow<Wry> {
        app.get_webview_window("main").unwrap()
    }

    pub fn create(app: &tauri::AppHandle<Wry>) -> Result<()> {
        settings::create(app)?;

        Ok(())
    }

    pub fn show(app: &tauri::AppHandle<Wry>, label: &str) -> Result<()> {
        if app.get_webview_window(label).is_none() {
            match label {
                labels::SETTINGS => settings::create(app)?,
                _ => {}
            }
        }

        let window = app.get_webview_window(label).unwrap();

        window.show()?;
        window.unminimize()?;
        window.set_focus()?;

        Ok(())
    }

    pub fn hide(app: &tauri::AppHandle<Wry>, label: &str) -> Result<()> {
        if let Some(window) = app.get_webview_window(label) {
            window.hide()?;
        }

        Ok(())
    }

    pub fn close(app: &tauri::AppHandle<Wry>, label: &str) -> Result<()> {
        if let Some(window) = app.get_webview_window(label) {
            window.close()?;
        }

        Ok(())
    }

    pub fn exists(app: &tauri::AppHandle<Wry>, label: &str) -> bool {
        app.get_webview_window(label).is_some()
    }
}

impl StartupService {
    pub fn apply(app: &AppHandle<Wry>, launch: bool) -> Result<()> {
        #[cfg(debug_assertions)]
        {
            return Ok(());
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
