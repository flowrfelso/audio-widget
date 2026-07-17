use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioChangedEvent {
    pub volume: u8,
    pub muted: bool,
}