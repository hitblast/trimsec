use std::collections::HashSet;

use ureq::Agent;

use crate::{
    core::{
        api::types::{YTPlaylistItems, YTPlaylistList, YTVideos, YTVideosItem},
        time::TDuration,
        youtils::YoutubeId,
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

pub struct ApiClient<'a> {
    client: Agent,
    key: &'a str,
}

impl<'a> ApiClient<'a> {
    #[must_use]
    pub fn new(key: &'a str) -> Self {
        Self {
            client: Agent::new_with_defaults(),
            key,
        }
    }

    /// Returns a vector of IDs from a single YouTube ID.
    ///
    /// This is expected to be used for fetching the contents of a playlist (or "video IDs"). If the [`YoutubeId`] object
    /// is not a playlist, then a vector would be returned with the ID that was originally passed in.
    pub fn expand_id(
        &self,
        id: &YoutubeId,
        max_items: usize,
    ) -> Result<Vec<String>, TYoutubeError> {
        let total_ids = {
            let mut next_tok: Option<String> = None;
            let mut ids = Vec::new();
            let mut seen_tokens: HashSet<String> = HashSet::new();

            if id.is_playlist() {
                let response: YTPlaylistList = self
                    .client
                    .get(PLAYLISTS_URL)
                    .query_pairs([
                        ("part", "contentDetails"),
                        ("id", id.id()),
                        ("key", self.key),
                        ("maxResults", "1"),
                    ])
                    .call()
                    .map_err(TYoutubeError::UreqError)?
                    .body_mut()
                    .read_json()
                    .map_err(|_| TYoutubeError::ResponseBodyParseFailure)?;

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
                    let max_results = (traversible_items - start).min(50).to_string();

                    let mut query_pairs = Vec::from([
                        ("playlistId", id.id()),
                        ("key", self.key),
                        ("maxResults", &max_results),
                        ("part", "contentDetails"),
                    ]);

                    if let Some(ref tok) = next_tok {
                        query_pairs.push(("pageToken", tok))
                    };

                    let response: YTPlaylistItems = self
                        .client
                        .get(PLAYLISTITEMS_URL)
                        .query_pairs(query_pairs)
                        .call()
                        .map_err(TYoutubeError::UreqError)?
                        .body_mut()
                        .read_json()
                        .map_err(|_| TYoutubeError::ResponseBodyParseFailure)?;

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
            let mut response: YTVideos = self
                .client
                .get(VIDEOS_URL)
                .query_pairs([
                    ("id", chunk_ids.join(",").as_str()),
                    ("key", self.key),
                    ("part", "snippet,contentDetails"),
                ])
                .call()
                .map_err(TYoutubeError::UreqError)?
                .body_mut()
                .read_json()
                .map_err(|_| TYoutubeError::ResponseBodyParseFailure)?;

            vector.append(&mut response.items);
        }

        Ok(vector)
    }

    /// Fetches the total duration from a single YouTube ID. The ID could be of either a playlist or a video.
    pub fn fetch_duration_from_id(
        &self,
        id: &YoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYoutubeError> {
        let total_ids = self.expand_id(id, max_items)?;
        let fetched_items = self.fetch_video_items(&total_ids)?;

        let total_duration: TDuration = fetched_items
            .into_iter()
            .map(|f| {
                TDuration::parse_str(
                    f.content_details
                        .duration
                        .to_lowercase()
                        .trim_start_matches("pt"),
                )
                .unwrap_or_default()
            })
            .collect::<Vec<TDuration>>()
            .into_iter()
            .sum();

        Ok(total_duration)
    }
}
