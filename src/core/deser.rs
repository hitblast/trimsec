use serde::Deserialize;

// videos

#[derive(Debug, Deserialize)]
pub struct YTVideos {
    pub items: Vec<YTVideosItem>,
}

#[derive(Debug, Deserialize)]
pub struct YTVideosItem {
    pub snippet: YTVideoSnippet,
    #[serde(rename = "contentDetails")]
    pub content_details: YTVideosContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTVideoSnippet {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct YTVideosContentDetails {
    pub duration: String,
}

// playlists

#[derive(Debug, Deserialize)]
pub struct YTPlaylistList {
    pub items: Vec<YTPlaylistListItem>,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistListItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YTPlaylistListItemContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistListItemContentDetails {
    #[serde(rename = "itemCount")]
    pub item_count: usize,
}

// playlistItems

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItems {
    pub items: Vec<YTPlaylistItemsItem>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItemsItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YTPlaylistItemsItemContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItemsItemContentDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
}
