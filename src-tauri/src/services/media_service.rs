use std::sync::atomic::AtomicI64;

use anyhow::Result;

use crate::{
    events::{MediaChanged, PlaybackState, TimelineState},
    platform::p_windows::media::manager::MediaManager,
};

pub struct MediaService {
    manager: MediaManager,
}

impl MediaService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            manager: MediaManager::new().await?,
        })
    }

    pub async fn play(&self) -> Result<PlaybackState> {
        self.manager.play().await
    }

    pub async fn pause(&self) -> Result<PlaybackState> {
        self.manager.pause().await
    }

    pub async fn toggle_play_pause(&self) -> Result<PlaybackState> {
        self.manager.toggle_play_pause().await
    }

    pub async fn previous(&self) -> Result<()> {
        self.manager.previous().await
    }

    pub async fn next(&self) -> Result<()> {
        self.manager.next().await
    }

    pub async fn seek_forward_10(&self) -> Result<()> {
        self.manager.seek_relative(10).await
    }

    pub async fn seek_forward_30(&self) -> Result<()> {
        self.manager.seek_relative(30).await
    }

    pub async fn seek_back_10(&self) -> Result<()> {
        self.manager.seek_relative(-10).await
    }

    pub async fn seek_back_30(&self) -> Result<()> {
        self.manager.seek_relative(-30).await
    }

    pub async fn current_info(&self) -> Result<MediaChanged> {
        self.manager.current_info().await
    }
}
