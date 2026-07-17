use tauri::{menu::MenuEvent, AppHandle, Manager, Wry};

use crate::{
    services::window_service::WindowService, ui::settings::repository::SettingsRepository,
    AppManager,
};

use super::{ids::*, menu::TrayMenu};

pub fn on_menu_event(
    tray: TrayMenu,
) -> impl Fn(&AppHandle<Wry>, MenuEvent) + Send + Sync + 'static {
    move |app, event| match event.id().as_ref() {
        PIN => {
            let _ = AppManager::toggle_pin(app, &tray);
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
