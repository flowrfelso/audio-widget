use anyhow::Result;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent, Wry};

use super::labels;

pub fn create(app: &tauri::AppHandle<Wry>) -> Result<()> {
    // if app.get_webview_window(labels::SETTINGS).is_some() {
    //     return Ok(());
    // }

    if let Some(window) = app.get_webview_window(labels::SETTINGS) {
        window.show()?;
        window.set_focus()?;
        return Ok(());
    }

    let window =
        WebviewWindowBuilder::new(app, labels::SETTINGS, WebviewUrl::App("settings".into()))
            .title("Settings")
            .visible(false)
            .center()
            .inner_size(560.0, 420.0)
            .resizable(false)
            .maximizable(false)
            .build()?;

    let window_for_event = window.clone();

    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            // Sử dụng bản clone bên trong closure
            let _ = window_for_event.hide();
        }
    });

    Ok(())
}
