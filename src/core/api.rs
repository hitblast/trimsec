use reqwest::blocking::Client;

use crate::{
    core::{
        deser::{YTPlaylist, YTVideos},
        time::{parse_duration, parse_time},
    },
    errors::TYoutubeError,
    youtube_utils::YoutubeId,
};

pub struct ApiClientManager<'a> {
    client: Client,
    key: &'a str,
}

impl<'a> ApiClientManager<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            client: Client::new(),
            key: key,
        }
    }

    pub fn fetch_duration_from_id(&self, id: &YoutubeId) -> Result<String, TYoutubeError> {
        let final_identifier = if id.is_playlist {
            let url = format!(
                "https://www.googleapis.com/youtube/v3/playlistItems?playlistId={}&key={}&part=contentDetails",
                id.id, self.key
            );

            let response: YTPlaylist = self
                .client
                .get(url)
                .send()
                .map_err(|e| TYoutubeError::Reqwest(e))?
                .json()
                .map_err(|e| TYoutubeError::Reqwest(e))?;

            let ids = response
                .items
                .iter()
                .map(|f| f.content_details.video_id.clone())
                .collect::<Vec<String>>()
                .join(",");
            ids
        } else {
            id.id.clone()
        };

        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails",
            final_identifier, self.key
        );

        let response: YTVideos = self
            .client
            .get(url)
            .send()
            .map_err(|e| TYoutubeError::Reqwest(e))?
            .json()
            .map_err(|e| TYoutubeError::Reqwest(e))?;

        let duration_int = response
            .items
            .iter()
            .map(|f| {
                let (dur, _) = parse_duration(
                    &f.content_details
                        .duration
                        .to_lowercase()
                        .trim_start_matches("pt"),
                )
                .unwrap();
                dur
            })
            .collect::<Vec<f64>>()
            .iter()
            .sum();
        let duration_str = parse_time(duration_int);

        Ok(duration_str)
    }
}
