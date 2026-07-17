mod events;
mod ids;
pub mod menu;

use tauri::{tray::TrayIconBuilder, App, Manager};

use crate::{state::tray::TrayState, ui::tray::menu::TrayMenu};

pub fn create(app: &App) -> tauri::Result<TrayMenu> {
    let tray = menu::TrayMenu::new(app)?;

    app.manage(TrayState {
        pin: tray.pin.clone(),
    });

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray.menu)
        .on_menu_event(events::on_menu_event(tray.clone()))
        .build(app)?;

    Ok(tray)
}
