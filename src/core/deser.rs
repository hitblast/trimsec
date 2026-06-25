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
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistItem {
    #[serde(rename = "contentDetails")]
    pub content_details: YTPlaylistContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTPlaylistContentDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
}
