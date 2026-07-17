use crate::{
    events::{MediaChanged, PlaybackState},
    services::media_service::MediaService,
};

#[tauri::command]
pub async fn play() -> Result<PlaybackState, String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .play()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause() -> Result<PlaybackState, String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .pause()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_play_pause() -> Result<PlaybackState, String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .toggle_play_pause()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn previous() -> anyhow::Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .previous()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn next() -> Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .next()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seek_forward_10() -> anyhow::Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .seek_forward_10()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seek_forward_30() -> anyhow::Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .seek_forward_30()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seek_back_10() -> anyhow::Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .seek_back_10()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seek_back_30() -> anyhow::Result<(), String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .seek_back_30()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn current_media_info() -> Result<MediaChanged, String> {
    MediaService::new()
        .await
        .map_err(|e| e.to_string())?
        .current_info()
        .await
        .map_err(|e| e.to_string())
}
