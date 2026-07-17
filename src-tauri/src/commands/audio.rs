use crate::services::audio_service::AudioService;

#[tauri::command]
pub fn get_volume() -> Result<u8, String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .volume()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_volume(volume: u8) -> Result<(), String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .set_volume(volume)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn volume_up(step: u8) -> Result<(), String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .volume_up(step)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn volume_down(step: u8) -> Result<(), String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .volume_down(step)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_mute() -> Result<(), String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .toggle_mute()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_muted() -> Result<bool, String> {
    AudioService::new()
        .map_err(|e| e.to_string())?
        .muted()
        .map_err(|e| e.to_string())
}
