use tauri::{menu::MenuEvent, AppHandle, Manager, Wry};

use crate::state::WindowState;

use super::{ids::*, menu::TrayMenu};

pub fn on_menu_event(
    tray: TrayMenu,
) -> impl Fn(&AppHandle<Wry>, MenuEvent) + Send + Sync + 'static {
    move |app, event| match event.id().as_ref() {
        PIN => {
            let state = app.state::<WindowState>();

            let mut pinned = state.pinned.lock().unwrap();

            *pinned = !*pinned;

            let _ = tray.pin.set_checked(*pinned);

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(*pinned);
            }
        }

        MEDIA => {}

        SETTINGS => {}

        RESTART => {
            app.restart();
        }

        EXIT => {
            app.exit(0);
        }

        _ => {}
    }
}
