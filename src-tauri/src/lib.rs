mod commands;
mod events;
mod platform;
mod services;
mod state;
mod ui;

use tauri::Manager;

use crate::commands::{audio::*, media::*};

use platform::p_windows::audio::AudioListener;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // hardcode
            if let Some(window) = app.get_webview_window("main") {
                window.set_always_on_top(true)?;
            }

            ui::tray::create(app)?;

            let manager = AudioListener::start(app.handle().clone())?;

            app.manage(state::AudioState {
                manager: std::sync::Mutex::new(manager),
            });

            app.manage(state::WindowState {
                pinned: std::sync::Mutex::new(true),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_volume,
            set_volume,
            volume_up,
            volume_down,
            toggle_mute,
            is_muted,
            play,
            pause,
            toggle_play_pause,
            previous,
            next,
            seek_forward_10,
            seek_forward_30,
            seek_back_10,
            seek_back_30,
            current_media_info
        ])
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
