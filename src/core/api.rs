use std::collections::HashSet;

use reqwest::blocking::Client;

use crate::{
    core::{
        deser::{YTPlaylistItems, YTPlaylistList, YTVideos},
        time::parse_duration,
    },
    errors::TYoutubeError,
    youtube_utils::YoutubeId,
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

    fn fetch_playlist_traversible_count(
        &self,
        playlist_id: &str,
        given_max: usize,
    ) -> Result<usize, TYoutubeError> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlists?part=contentDetails&id={}&key={}&maxResults=1",
            playlist_id, self.key
        );

        let response: YTPlaylistList = self
            .client
            .get(url)
            .send()
            .map_err(TYoutubeError::Reqwest)?
            .json()
            .map_err(TYoutubeError::Reqwest)?;

        if let Some(ic) = response.items.first() {
            let max_traversible = ic.content_details.item_count;

            if given_max != 0 {
                if given_max > max_traversible {
                    Err(TYoutubeError::InvalidMaxSize((given_max, max_traversible)))
                } else {
                    Ok(given_max)
                }
            } else {
                Ok(max_traversible)
            }
        } else {
            Err(TYoutubeError::InvalidPlaylist(playlist_id.to_string()))
        }
    }

    fn fetch_playlist_items_page(
        &self,
        playlist_id: &str,
        start: usize,
        traversible_items: usize,
        next_tok: Option<&String>,
    ) -> Result<YTPlaylistItems, TYoutubeError> {
        let max_results = (traversible_items - start).min(50);

        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlistItems?playlistId={}&key={}&maxResults={}&part=contentDetails{}",
            playlist_id,
            self.key,
            max_results,
            if let Some(tok) = next_tok {
                format!("&pageToken={tok}")
            } else {
                "".to_string()
            }
        );

        self.client
            .get(url)
            .send()
            .map_err(TYoutubeError::Reqwest)?
            .json()
            .map_err(TYoutubeError::Reqwest)
    }

    fn fetch_playlist_video_ids(
        &self,
        playlist_id: &str,
        traversible_items: usize,
    ) -> Result<Vec<String>, TYoutubeError> {
        let mut next_tok: Option<String> = None;
        let mut ids = Vec::new();
        let mut seen_tokens: HashSet<String> = HashSet::new();

        for start in (0..traversible_items).step_by(50) {
            let response = self.fetch_playlist_items_page(
                playlist_id,
                start,
                traversible_items,
                next_tok.as_ref(),
            )?;

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

        Ok(ids)
    }

    fn resolve_video_ids(
        &self,
        id: &YoutubeId,
        given_max: usize,
    ) -> Result<Vec<String>, TYoutubeError> {
        if id.is_playlist {
            let traversible_items = self.fetch_playlist_traversible_count(&id.id, given_max)?;
            self.fetch_playlist_video_ids(&id.id, traversible_items)
        } else {
            Ok(vec![id.id.clone()])
        }
    }

    fn fetch_chunk_duration(&self, chunk_ids: &[String]) -> Result<f64, TYoutubeError> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=contentDetails",
            chunk_ids.join(","),
            self.key
        );

        let response: YTVideos = self
            .client
            .get(url)
            .send()
            .map_err(TYoutubeError::Reqwest)?
            .json()
            .map_err(TYoutubeError::Reqwest)?;

        let chunk_duration: f64 = response
            .items
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

        Ok(chunk_duration)
    }

    fn fetch_total_duration(&self, total_ids: &[String]) -> Result<f64, TYoutubeError> {
        let mut total_duration: f64 = 0.0;

        for chunk_ids in total_ids.chunks(50) {
            total_duration += self.fetch_chunk_duration(chunk_ids)?;
        }

        Ok(total_duration)
    }

    pub fn fetch_duration_from_id(
        &self,
        id: &YoutubeId,
        given_max: usize,
    ) -> Result<(f64, usize), TYoutubeError> {
        let total_ids = self.resolve_video_ids(id, given_max)?;
        let total_duration = self.fetch_total_duration(&total_ids)?;

        Ok((total_duration, total_ids.len()))
    }
}
