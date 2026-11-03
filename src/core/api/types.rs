use serde::Deserialize;

// The naming convention is based on the actual API endpoints they are deserialized from.

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
