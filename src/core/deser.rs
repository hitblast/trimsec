use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct YTVideos {
    pub items: Vec<YTVideosItem>,
}

#[derive(Debug, Deserialize)]
pub struct YTVideosItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YTVideosContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTVideosContentDetails {
    pub duration: String,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylist {
    pub items: Vec<YTPlaylistItem>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YTPlaylistItemContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItemContentDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
}
