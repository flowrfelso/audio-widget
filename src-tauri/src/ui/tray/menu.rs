use tauri::{
    menu::{CheckMenuItem, CheckMenuItemBuilder, Menu, MenuBuilder, MenuItem, MenuItemBuilder},
    App, Wry,
};

use super::ids::*;

pub struct TrayMenu {
    pub menu: Menu<Wry>,

    pub pin: CheckMenuItem<Wry>,

    pub media: MenuItem<Wry>,

    pub settings: MenuItem<Wry>,

    pub restart: MenuItem<Wry>,

    pub exit: MenuItem<Wry>,
}

impl TrayMenu {
    pub fn new(app: &App) -> tauri::Result<Self> {
        let pin = CheckMenuItemBuilder::with_id(PIN, "Pin on Top")
            .checked(true)
            .build(app)?;

        let media = MenuItemBuilder::with_id(MEDIA, "Current Media")
            .enabled(false)
            .build(app)?;

        let settings = MenuItemBuilder::with_id(SETTINGS, "Settings").build(app)?;

        let restart = MenuItemBuilder::with_id(RESTART, "Restart").build(app)?;

        let exit = MenuItemBuilder::with_id(EXIT, "Exit").build(app)?;

        let menu = MenuBuilder::new(app)
            .text("title", "Audio Widget")
            .separator()
            .item(&pin)
            .item(&media)
            .separator()
            .item(&settings)
            .item(&restart)
            .separator()
            .item(&exit)
            .build()?;

        Ok(Self {
            menu,
            pin,
            media,
            settings,
            restart,
            exit,
        })
    }
}
