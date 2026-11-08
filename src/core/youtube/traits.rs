use enum_dispatch::enum_dispatch;

use crate::{
    core::{
        time::TDuration,
        youtube::youtils::{TYoutubeId, TYoutubeVideo},
    },
    errors::TYtClientError,
};

/// Represents a YouTube client's common methods inside trimsec.
///
/// This trait allows to switch between `yt-dlp` and the internal YouTube Data API wrapper via static dispatch
/// and is shared between both the upper and the lower-level structs.
///
#[enum_dispatch]
pub trait YtClientCommon: Send + Sync {
    /// Returns a vector of IDs from a single YouTube ID.
    ///
    /// This is expected to be used for fetching the contents of a playlist (or "video IDs"). If the [`YoutubeId`] object
    /// is not a playlist, then a vector would be returned with the ID that was originally passed in.
    fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeId>, TYtClientError>;

    /// Fetches an array of videos by their IDs (represented by [`TYoutubeId`]).
    fn fetch_video_items(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeVideo>, TYtClientError>;

    /// Fetches the total duration from a single YouTube ID. The ID could be of either a playlist or a video.
    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYtClientError>;
}
