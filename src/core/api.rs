use std::collections::HashSet;

use reqwest::blocking::Client;

use crate::{
    core::{
        deser::{YTPlaylistItems, YTVideos},
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

    pub fn fetch_duration_from_id(
        &self,
        id: YoutubeId,
        max_items: usize,
    ) -> Result<String, TYoutubeError> {
        let total_ids = {
            let mut next_tok: Option<String> = None;
            let mut ids = Vec::new();
            let mut seen_tokens: HashSet<String> = HashSet::new();

            if id.is_playlist {
                loop {
                    if max_items != 0 && ids.len() >= max_items {
                        break;
                    }

                    let url = format!(
                        "https://www.googleapis.com/youtube/v3/playlistItems?playlistId={}&key={}&maxResults=50&part=contentDetails{}",
                        id.id,
                        self.key,
                        if let Some(ref tok) = next_tok {
                            format!("&pageToken={tok}")
                        } else {
                            "".to_string()
                        }
                    );

                    let response: YTPlaylistItems = self
                        .client
                        .get(url)
                        .send()
                        .map_err(|e| TYoutubeError::Reqwest(e))?
                        .json()
                        .map_err(|e| TYoutubeError::Reqwest(e))?;

                    if let Some(t) = &response.next_page_token
                        && seen_tokens.contains(t)
                    {
                        break;
                    }

                    let current_ids = response
                        .items
                        .into_iter()
                        .map(|f| f.content_details.video_id);

                    ids.extend(current_ids);

                    if let Some(new_tok) = response.next_page_token {
                        next_tok = Some(new_tok.clone());
                        seen_tokens.insert(new_tok);
                    } else {
                        break;
                    }
                }
            } else {
                ids.push(id.id);
            }

            ids
        };

        let mut total_duration: f64 = 0.0;

        for chunk_ids in total_ids.chunks(50) {
            let url = format!(
                "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails",
                chunk_ids.join(","),
                self.key
            );

            let response: YTVideos = self
                .client
                .get(url)
                .send()
                .map_err(|e| TYoutubeError::Reqwest(e))?
                .json()
                .map_err(|e| TYoutubeError::Reqwest(e))?;

            let chunk_duration: f64 = response
                .items
                .iter()
                .map(|f| {
                    let (dur, _) = parse_duration(
                        &f.content_details
                            .duration
                            .to_lowercase()
                            .trim_start_matches("pt"),
                    )
                    .unwrap_or((0.0, 0));
                    dur
                })
                .collect::<Vec<f64>>()
                .iter()
                .sum();

            total_duration += chunk_duration;
        }

        Ok(parse_time(total_duration))
    }
}
