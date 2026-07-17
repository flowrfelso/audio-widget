use anyhow::Result;
use std::sync::atomic::{AtomicI64, Ordering};

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};
// use windows_future::IAsyncOperation;

use crate::events::{media::MediaChanged, PlaybackState};

pub struct MediaManager {
    manager: GlobalSystemMediaTransportControlsSessionManager,
    last_position_ticks: AtomicI64,
}

impl MediaManager {
    pub async fn new() -> Result<Self> {
        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;

        Ok(Self {
            manager,
            last_position_ticks: AtomicI64::new(0),
        })
    }

    fn current_session(&self) -> Result<GlobalSystemMediaTransportControlsSession> {
        Ok(self.manager.GetCurrentSession()?)
    }

    pub async fn next(&self) -> Result<()> {
        let session = self.current_session()?;

        // let media = session.TryGetMediaPropertiesAsync()?.await?;

        let controls = session.GetPlaybackInfo()?.Controls()?;

        if !controls.IsNextEnabled()? {
            anyhow::bail!("Next command is disabled");
        }

        let _ = session.TrySkipNextAsync()?.await;

        // let success = op.await?;

        Ok(())
    }

    pub async fn previous(&self) -> Result<()> {
        let session = self.current_session()?;

        let controls = session.GetPlaybackInfo()?.Controls()?;

        if !controls.IsPreviousEnabled()? {
            anyhow::bail!("Previous command is disabled");
        }

        session.TrySkipPreviousAsync()?.await?;

        Ok(())
    }

    pub async fn play(&self) -> Result<PlaybackState> {
        let session = self.current_session()?;

        let success = session.TryPlayAsync()?.await?;

        if !success {
            anyhow::bail!("Play failed");
        }

        let info = session.GetPlaybackInfo()?;
        Ok((PlaybackState {
            playing: info.PlaybackStatus()?
                == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing,
        }))
    }

    pub async fn pause(&self) -> Result<PlaybackState> {
        let session = self.current_session()?;

        let success = session.TryPauseAsync()?.await?;

        if !success {
            anyhow::bail!("Pause failed");
        }

        let info = session.GetPlaybackInfo()?;
        Ok((PlaybackState {
            playing: info.PlaybackStatus()?
                == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing,
        }))
    }

    pub async fn toggle_play_pause(&self) -> Result<PlaybackState> {
        let session = self.current_session()?;
        let info = session.GetPlaybackInfo()?;
        let is_playing = info.PlaybackStatus()?
            == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing;

        let success = if is_playing {
            session.TryPauseAsync()?.await?
        } else {
            session.TryPlayAsync()?.await?
        };

        // anyhow::ensure!(
        //     success,
        //     "Command failed, media session did not accept the request"
        // );

        if !success {
            anyhow::bail!("Command failed, media session did not accept the request");
        }

        Ok(PlaybackState {
            playing: !is_playing,
        })
    }

    pub async fn current_info(&self) -> Result<MediaChanged> {
        let session = self.current_session()?;
        let media = session.TryGetMediaPropertiesAsync()?.await?;

        let playback = session.GetPlaybackInfo()?;

        let controls = playback.Controls()?;

        Ok(MediaChanged {
            title: media.Title()?.to_string(),
            artist: media.Artist()?.to_string(),
            playing: playback.PlaybackStatus()?
                == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing,
            source: session.SourceAppUserModelId()?.to_string(),
            can_next: controls.IsNextEnabled()?,
            can_previous: controls.IsPreviousEnabled()?,
        })
    }

    // pub async fn seek_relative(&self, offset_seconds: i64) -> Result<()> {
    //     let session = self.current_session()?;

    //     let timeline = session.GetTimelineProperties()?;

    //     let current = timeline.Position()?.Duration;

    //     let end = timeline.EndTime()?.Duration;

    //     let target =
    //         (current + offset_seconds * 10_000_000)
    //             .clamp(0, end);

    //     let ok = session
    //         .TryChangePlaybackPositionAsync(target)?
    //         .await?;

    //     if !ok {
    //         anyhow::bail!("Seek failed");
    //     }

    //     Ok(())
    // }

    // pub async fn seek_relative(
    //     &self,
    //     offset_ms: i64,
    // ) -> Result<TimelineState> {
    //     let session = self.current_session()?;

    //     let target = {
    //         let mut timeline = self.timeline.lock().unwrap();

    //         let current = timeline.position_ms as i64;

    //         let duration = timeline.duration_ms as i64;

    //         let target =
    //             (current + offset_ms)
    //                 .clamp(0, duration);

    //         timeline.position_ms = target as u64;

    //         timeline.clone()
    //     };

    //     let ok = session
    //         .TryChangePlaybackPositionAsync(
    //             (target.position_ms * 10_000) as i64,
    //         )?
    //         .await?;

    //     if !ok {
    //         anyhow::bail!("Seek failed");
    //     }

    //     Ok(target)
    // }

    // pub async fn seek_absolute(
    //     &self,
    //     position_ms: u64,
    // ) -> Result<TimelineState> {
    //     let session = self.current_session()?;

    //     let target = {
    //         let mut timeline = self.timeline.lock().unwrap();

    //         timeline.position_ms =
    //             position_ms.min(timeline.duration_ms);

    //         timeline.clone()
    //     };

    //     session
    //         .TryChangePlaybackPositionAsync(
    //             (target.position_ms * 10_000) as i64,
    //         )?
    //         .await?;

    //     Ok(target)
    // }

    pub async fn seek_relative(&self, offset_seconds: i64) -> Result<()> {
        let session = self.current_session()?;

        // 1. Lấy vị trí từ hệ thống
        let timeline = session.GetTimelineProperties()?;
        let actual_pos = timeline.Position()?.Duration;

        // 2. Lấy vị trí cache (nếu lệnh trước đó vừa seek xong)
        let cached_pos = self.last_position_ticks.load(Ordering::SeqCst);

        // 3. Quyết định dùng cái nào làm gốc:
        // Nếu chênh lệch quá lớn (do user vừa seek hoặc tua tay), dùng actual_pos.
        // Nếu vừa mới seek xong, dùng cached_pos làm gốc để cộng dồn.
        let base_pos = if (actual_pos - cached_pos).abs() < 2_000_000 {
            // chênh lệch < 0.2s
            cached_pos
        } else {
            actual_pos
        };

        let end = timeline.EndTime()?.Duration;
        let target = (base_pos + offset_seconds * 10_000_000).clamp(0, end);

        // 4. Update cache ngay lập tức trước khi gọi async
        self.last_position_ticks.store(target, Ordering::SeqCst);

        let ok = session.TryChangePlaybackPositionAsync(target)?.await?;

        if !ok {
            // Nếu fail, hoàn tác cache
            self.last_position_ticks.store(actual_pos, Ordering::SeqCst);
            anyhow::bail!("Seek failed");
        }

        Ok(())
    }
}
