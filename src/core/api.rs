use std::collections::HashSet;

use reqwest::blocking::Client;

use crate::{
    core::{
        deser::{YTPlaylistItems, YTPlaylistList, YTVideos, YTVideosItem},
        time::parse_duration,
        youtils::YoutubeId,
    },
    errors::TYoutubeError,
};

pub struct ApiClientManager<'a> {
    client: Client,
    key: &'a str,
}

impl<'a> ApiClientManager<'a> {
    #[must_use]
    pub fn new(key: &'a str) -> Self {
        Self {
            client: Client::new(),
            key,
        }
    }

    /// Returns a vector of IDs from a single YouTube ID.
    ///
    /// This is expected to be used in the case where the user passes a "playlist ID"
    /// and a certain amount of items' identities need to be known.
    fn fetch_ids_from_id(
        &self,
        id: &YoutubeId,
        max_items: usize,
    ) -> Result<Vec<String>, TYoutubeError> {
        let total_ids = {
            let mut next_tok: Option<String> = None;
            let mut ids = Vec::new();
            let mut seen_tokens: HashSet<String> = HashSet::new();

            if id.is_playlist {
                let url = format!(
                    "https://www.googleapis.com/youtube/v3/playlists?part=contentDetails&id={}&key={}&maxResults=1",
                    &id.id, self.key
                );

                let response: YTPlaylistList = self
                    .client
                    .get(url)
                    .send()
                    .map_err(TYoutubeError::Reqwest)?
                    .json()
                    .map_err(TYoutubeError::Reqwest)?;

                let traversible_items = if let Some(ic) = response.items.first() {
                    let max_traversible = ic.content_details.item_count;

                    if max_items != 0 {
                        if max_items > max_traversible {
                            return Err(TYoutubeError::InvalidMaxSize((
                                max_items,
                                max_traversible,
                            )));
                        } else {
                            max_items
                        }
                    } else {
                        max_traversible
                    }
                } else {
                    return Err(TYoutubeError::InvalidPlaylist(id.id.clone()));
                };

                for start in (0..traversible_items).step_by(50) {
                    let max_results = (traversible_items - start).min(50);

                    let url = format!(
                        "https://www.googleapis.com/youtube/v3/playlistItems?playlistId={}&key={}&maxResults={}&part=contentDetails{}",
                        &id.id,
                        self.key,
                        max_results,
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
                        .map_err(TYoutubeError::Reqwest)?
                        .json()
                        .map_err(TYoutubeError::Reqwest)?;

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
                ids.push(id.id.clone());
            }

            ids
        };

        Ok(total_ids)
    }

    fn fetch_video_items(&self, ids: &[String]) -> Result<Vec<YTVideosItem>, TYoutubeError> {
        let mut vector: Vec<YTVideosItem> = Vec::new();

        for chunk_ids in ids.chunks(50) {
            let url = format!(
                "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails",
                chunk_ids.join(","),
                self.key
            );

            let mut response: YTVideos = self
                .client
                .get(url)
                .send()
                .map_err(TYoutubeError::Reqwest)?
                .json()
                .map_err(TYoutubeError::Reqwest)?;

            vector.append(&mut response.items);
        }

        Ok(vector)
    }

    pub fn fetch_duration_from_id(
        &self,
        id: &YoutubeId,
        max_items: usize,
    ) -> Result<(f64, usize), TYoutubeError> {
        let total_ids = self.fetch_ids_from_id(id, max_items)?;
        let fetched_items = self.fetch_video_items(&total_ids)?;

        let total_duration: f64 = fetched_items
            .iter()
            .map(|f| {
                let (dur, _) = parse_duration(
                    f.content_details
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

        Ok((total_duration, total_ids.len()))
    }
}
