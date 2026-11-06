use std::path::Path;

use http_cache_ureq::RedbManager;
use serde::Deserialize;

use crate::{
    core::{time::TDuration, youtils::TYoutubeId},
    errors::{TYtClientError, TYtInnerClientInitError},
};

// The naming convention is based on the actual API endpoints they are deserialized from.

pub trait YtInnerClient: Send + Sync {
    fn new(key: impl ToString) -> Result<Self, TYtInnerClientInitError>
    where
        Self: Sized;

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
    fn fetch_video_items(&self, ids: &[TYoutubeId]) -> Result<Vec<YtVideo>, TYtClientError>;

    /// Fetches the total duration from a single YouTube ID. The ID could be of either a playlist or a video.
    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYtClientError>;

    /// Returns the API-request cache-path for trimsec.
    ///
    /// This might return `None` if the client is not using the cache.
    ///
    /// NOTE: This is only available on the [`YtClient`] implementation.
    fn cache_path(&self) -> Option<&Path>;

    /// Returns the cache manager for the API client.
    ///
    /// This might return `None` if the client is not using the cache.
    ///
    /// NOTE: This is only available on the [`YtClient`] implementation.
    fn cache_manager(&self) -> Option<&RedbManager>;
}

// videos

#[derive(Debug, Deserialize)]
pub struct YtVideoCollection {
    pub items: Vec<YtVideo>,
}

#[derive(Debug, Deserialize)]
pub struct YtVideo {
    pub snippet: YtVideoSnippet,
    #[serde(rename = "contentDetails")]
    pub content_details: YtVideoContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YtVideoSnippet {
    #[allow(unused)]
    pub title: String,
    #[allow(unused)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct YtVideoContentDetails {
    pub duration: String,
}

// playlists

#[derive(Debug, Deserialize)]
pub struct YtPlaylistCollection {
    pub items: Vec<YtPlaylist>,
}

#[derive(Debug, Deserialize)]
pub struct YtPlaylist {
    #[serde(rename = "contentDetails")]
    pub content_details: YtPlaylistContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YtPlaylistContentDetails {
    #[serde(rename = "itemCount")]
    pub item_count: usize,
}

// playlistItems

#[derive(Debug, Deserialize)]
pub struct YtPlaylistItemCollection {
    pub items: Vec<YtPlaylistItem>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct YtPlaylistItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YtPlaylistItemContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YtPlaylistItemContentDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
}
