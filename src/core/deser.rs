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
