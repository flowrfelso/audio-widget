mod events;
mod ids;
pub mod menu;

use tauri::{tray::TrayIconBuilder, App};

pub fn create(app: &App) -> tauri::Result<()> {
    let tray = menu::TrayMenu::new(app)?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray.menu)
        .on_menu_event(events::on_menu_event(tray))
        .build(app)?;

    Ok(())
}
