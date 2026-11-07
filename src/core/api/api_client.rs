use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use http_cache_ureq::{CachedAgent, RedbManager};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::de::DeserializeOwned;
use ureq::Agent;

use crate::{
    core::{
        CACHE_DIR,
        api::types::{
            YtClientCommon, YtPlaylistCollection, YtPlaylistItemCollection, YtVideo,
            YtVideoCollection,
        },
        time::TDuration,
        youtils::TYoutubeId,
    },
    errors::{TYtClientError, TYtClientInitError},
};

macro_rules! gen_url {
    ($x:ident, $y:expr) => {
        static $x: &'static str = concat!("https://www.googleapis.com/youtube/v3", $y);
    };
}

gen_url!(PLAYLISTS_URL, "/playlists");
gen_url!(PLAYLISTITEMS_URL, "/playlistItems");
gen_url!(VIDEOS_URL, "/videos");

pub enum ApiClientType {
    Cached(CachedAgent<RedbManager>),
    Uncached(Agent),
}

pub struct YtInnerDataApiClient {
    client: ApiClientType,
    cache_path: Option<PathBuf>,
    cache_manager: Option<RedbManager>,
    key: String,
}

const HTTP_CACHE_FILENAME: &str = "trimsec-http-cache.redb";

fn get_http_cache_path() -> Option<PathBuf> {
    CACHE_DIR.as_ref().map(|f| f.join(HTTP_CACHE_FILENAME))
}

impl YtClientCommon for YtInnerDataApiClient {
    fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeId>, TYtClientError> {
        let total_ids = {
            let mut next_tok: Option<String> = None;
            let mut ids: Vec<TYoutubeId> = Vec::new();
            let mut seen_tokens: HashSet<String> = HashSet::new();

            if id.is_playlist() {
                let url = format!(
                    "{}?part=contentDetails&id={}&key={}&maxResults=1",
                    PLAYLISTS_URL,
                    id.as_str(),
                    self.key
                );

                let response: YtPlaylistCollection = self.get(&url)?;

                let traversible_items = if let Some(ic) = response.items.first() {
                    let max_traversible = ic.content_details.item_count;

                    if max_items != 0 {
                        if max_items > max_traversible {
                            return Err(TYtClientError::InvalidMaxSize((
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
                    return Err(TYtClientError::InvalidPlaylist(id.to_string()));
                };

                for start in (0..traversible_items).step_by(50) {
                    let max_results = (traversible_items - start).min(50);

                    let url = if let Some(ref tok) = next_tok {
                        format!(
                            "{}?playlistId={}&key={}&maxResults={}&part=contentDetails&pageToken={}",
                            PLAYLISTITEMS_URL,
                            id.as_str(),
                            self.key,
                            max_results,
                            tok
                        )
                    } else {
                        format!(
                            "{}?playlistId={}&key={}&maxResults={}&part=contentDetails",
                            PLAYLISTITEMS_URL,
                            id.as_str(),
                            self.key,
                            max_results
                        )
                    };

                    let response: YtPlaylistItemCollection = self.get(&url)?;

                    if let Some(t) = &response.next_page_token
                        && seen_tokens.contains(t)
                    {
                        break;
                    }

                    let current_ids = response
                        .items
                        .into_iter()
                        .map(|f| TYoutubeId::new(f.content_details.video_id, false));

                    ids.extend(current_ids);

                    if let Some(new_tok) = response.next_page_token {
                        next_tok = Some(new_tok.clone());
                        seen_tokens.insert(new_tok);
                    } else {
                        break;
                    }
                }
            } else {
                ids.push(id.clone());
            }

            ids
        };

        Ok(total_ids)
    }

    fn fetch_video_items(&self, ids: &[TYoutubeId]) -> Result<Vec<YtVideo>, TYtClientError> {
        let mut vector: Vec<YtVideo> = Vec::new();

        let processed: Result<Vec<YtVideoCollection>, TYtClientError> = ids
            .chunks(50)
            .into_iter()
            .par_bridge()
            .into_par_iter()
            .map(|chunk_ids| {
                let url = format!(
                    "{}?id={}&key={}&part=snippet,contentDetails",
                    VIDEOS_URL,
                    chunk_ids
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<&str>>()
                        .join(","),
                    self.key
                );

                self.get::<YtVideoCollection>(&url)
            })
            .collect();

        for mut chunk in processed? {
            vector.append(&mut chunk.items);
        }

        Ok(vector)
    }

    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYtClientError> {
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

impl YtInnerDataApiClient {
    pub fn new(key: impl ToString) -> Result<Self, TYtClientInitError> {
        let mut cache_path = None;
        let mut cache_manager = None;

        let agent = match get_http_cache_path() {
            Some(path) => ApiClientType::Cached({
                let manager = RedbManager::new(&path)
                    .map_err(|e| TYtClientInitError::YtDataApi(e.to_string()))?;
                cache_path = Some(path);
                cache_manager = Some(manager.clone());

                CachedAgent::builder()
                    .cache_manager(manager)
                    .cache_mode(http_cache_ureq::CacheMode::Default)
                    .build()
                    .map_err(|e| TYtClientInitError::YtDataApi(e.to_string()))?
            }),
            _ => ApiClientType::Uncached(Agent::new_with_defaults()),
        };

        Ok(Self {
            client: agent,
            cache_path,
            cache_manager,
            key: key.to_string(),
        })
    }

    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, TYtClientError> {
        match &self.client {
            ApiClientType::Cached(cached_agent) => smol::block_on(async {
                cached_agent
                    .get(url)
                    .call()
                    .await
                    .map_err(|e| TYtClientError::UreqError(e.to_string()))?
                    .into_json::<T>()
                    .map_err(|e| TYtClientError::InvalidResponseBody(e.to_string()))
            }),
            ApiClientType::Uncached(agent) => agent
                .get(url)
                .call()
                .map_err(|e| TYtClientError::UreqError(e.to_string()))?
                .body_mut()
                .read_json()
                .map_err(|e| TYtClientError::InvalidResponseBody(e.to_string())),
        }
    }

    pub fn cache_path(&self) -> Option<&Path> {
        self.cache_path.as_ref().map(|f| f.as_path())
    }

    pub fn cache_manager(&self) -> Option<&RedbManager> {
        self.cache_manager.as_ref()
    }
}
