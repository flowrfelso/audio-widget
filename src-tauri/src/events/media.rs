use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct MediaChanged {
    pub title: String,
    pub artist: String,
    pub playing: bool,
    pub source: String,
    pub can_next: bool,
    pub can_previous: bool,
}

#[derive(Clone, Serialize)]
pub struct PlaybackState {
    pub playing: bool,
}

#[derive(Clone, Serialize)]
pub struct TimelineState {
    pub position_ms: u64,
    pub duration_ms: u64,
}
