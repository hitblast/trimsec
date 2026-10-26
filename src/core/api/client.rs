use std::collections::HashSet;

use http_cache_ureq::{CachedAgent, RedbManager};
use serde::de::DeserializeOwned;

use crate::{
    core::{
        api::types::{YTPlaylistItems, YTPlaylistList, YTVideos, YTVideosItem},
        time::TDuration,
        youtils::TYoutubeId,
    },
    errors::TYoutubeError,
};

macro_rules! gen_url {
    ($x:ident, $y:expr) => {
        static $x: &'static str = concat!("https://www.googleapis.com/youtube/v3", $y);
    };
}

gen_url!(PLAYLISTS_URL, "/playlists");
gen_url!(PLAYLISTITEMS_URL, "/playlistItems");
gen_url!(VIDEOS_URL, "/videos");

pub struct ApiClient {
    client: CachedAgent<RedbManager>,
    key: String,
}

impl<'a> ApiClient {
    #[must_use]
    pub fn new(key: impl ToString) -> Result<Self, anyhow::Error> {
        let agent = CachedAgent::builder()
            .cache_manager(RedbManager::new("./http-cache.redb").map_err(|e| anyhow::anyhow!(e))?)
            .cache_mode(http_cache_ureq::CacheMode::Default)
            .build()?;

        Ok(Self {
            client: agent,
            key: key.to_string(),
        })
    }

    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, TYoutubeError> {
        smol::block_on(async {
            self.client
                .get(url)
                .call()
                .await
                .map_err(|e| TYoutubeError::UreqError(e.to_string()))?
                .into_json::<T>()
                .map_err(|e| TYoutubeError::InvalidResponseBody(e.to_string()))
        })
    }

    /// Returns a vector of IDs from a single YouTube ID.
    ///
    /// This is expected to be used for fetching the contents of a playlist (or "video IDs"). If the [`YoutubeId`] object
    /// is not a playlist, then a vector would be returned with the ID that was originally passed in.
    pub fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<String>, TYoutubeError> {
        let total_ids = {
            let mut next_tok: Option<String> = None;
            let mut ids = Vec::new();
            let mut seen_tokens: HashSet<String> = HashSet::new();

            if id.is_playlist() {
                let url = format!(
                    "{}?part=contentDetails&id={}&key={}&maxResults=1",
                    PLAYLISTS_URL,
                    id.id(),
                    self.key
                );

                let response: YTPlaylistList = self.get(&url)?;

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
                    return Err(TYoutubeError::InvalidPlaylist(id.id().to_string()));
                };

                for start in (0..traversible_items).step_by(50) {
                    let max_results = (traversible_items - start).min(50);

                    let url = if let Some(ref tok) = next_tok {
                        format!(
                            "{}?playlistId={}&key={}&maxResults={}&part=contentDetails&pageToken={}",
                            PLAYLISTITEMS_URL,
                            id.id(),
                            self.key,
                            max_results,
                            tok
                        )
                    } else {
                        format!(
                            "{}?playlistId={}&key={}&maxResults={}&part=contentDetails",
                            PLAYLISTITEMS_URL,
                            id.id(),
                            self.key,
                            max_results
                        )
                    };

                    let response: YTPlaylistItems = self.get(&url)?;

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
                ids.push(id.id().to_string());
            }

            ids
        };

        Ok(total_ids)
    }

    /// Fetches a single chunk of video items.
    pub fn fetch_video_items(&self, ids: &[String]) -> Result<Vec<YTVideosItem>, TYoutubeError> {
        let mut vector: Vec<YTVideosItem> = Vec::new();

        for chunk_ids in ids.chunks(50) {
            let url = format!(
                "{}?id={}&key={}&part=snippet,contentDetails",
                VIDEOS_URL,
                chunk_ids.join(","),
                self.key
            );

            let mut response: YTVideos = self.get(&url)?;

            vector.append(&mut response.items);
        }

        Ok(vector)
    }

    /// Fetches the total duration from a single YouTube ID. The ID could be of either a playlist or a video.
    pub fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYoutubeError> {
        let total_ids = self.expand_id(id, max_items)?;
        let fetched_items = self.fetch_video_items(&total_ids)?;

        let total_duration: TDuration = fetched_items
            .into_iter()
            .map(|f| {
                f.content_details
                    .duration
                    .to_lowercase()
                    .trim_start_matches("pt")
                    .parse::<TDuration>()
                    .unwrap_or_default()
            })
            .collect::<Vec<TDuration>>()
            .into_iter()
            .sum();

        Ok(total_duration)
    }
}
