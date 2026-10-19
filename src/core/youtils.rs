use std::fmt::Display;

use anyhow::{Result as YoutilsResult, bail};
use url::Url;

use crate::core::config::Config;

pub fn decide_youtube_key(cfg: &Config) -> YoutilsResult<String> {
    const ENV_VAR_NAME: &str = "TRIMSEC_YOUTUBE_KEY";
    let x = std::env::var(ENV_VAR_NAME).ok();

    match x {
        Some(x) => Ok(x),
        None => match cfg.api_key() {
            Some(k) => Ok(k.to_string()),
            None => {
                bail!("API key not found! Set one using: ts key set [OPTIONS] <API_KEY>")
            }
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TYoutubeId {
    id: String,
    is_playlist: bool,
}

impl TYoutubeId {
    #[must_use]
    pub fn new(id: impl ToString, is_playlist: bool) -> Self {
        Self {
            id: id.to_string(),
            is_playlist,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
    #[must_use]
    pub fn is_playlist(&self) -> bool {
        self.is_playlist
    }
}

impl Display for TYoutubeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

#[must_use]
pub fn get_youtube_id(link: &str, force_playlist: bool) -> Option<TYoutubeId> {
    let mut is_playlist = false;

    let Ok(parsed_url) = Url::parse(link) else {
        return None;
    };

    if !parsed_url.host_str().is_some_and(|f| {
        f == "www.youtube.com" || f == "youtube.com" || f == "youtu.be" || f == "music.youtube.com"
    }) {
        return None;
    }

    let query_search = |q: &str, ptr: &str| {
        let x = q
            .split('&')
            .find(|p| p.starts_with(ptr))
            .map(|p| p.trim_start_matches(ptr).to_string());

        if x.as_deref().is_some_and(|f| !f.is_empty()) {
            x
        } else {
            None
        }
    };

    let id: Option<String> =
        if parsed_url.path().starts_with("/shorts/") || parsed_url.path().starts_with("/embed/") {
            parsed_url
                .path_segments()
                .and_then(|mut f| f.next_back())
                .map(|s| s.to_string())
        } else if parsed_url.path().starts_with("/watch") {
            parsed_url.query().and_then(|q| {
                if let Some(v) = query_search(q, "v=") {
                    if !force_playlist {
                        Some(v)
                    } else if let Some(list) = query_search(q, "list=") {
                        is_playlist = true;
                        Some(list)
                    } else {
                        None
                    }
                } else if let Some(list) = query_search(q, "list=") {
                    is_playlist = true;
                    Some(list)
                } else {
                    None
                }
            })
        } else if parsed_url.path().starts_with("/playlist") {
            is_playlist = true;
            parsed_url.query().and_then(|q| query_search(q, "list="))
        } else if parsed_url.host_str().is_some_and(|f| f == "youtu.be") {
            Some(parsed_url.path().trim_start_matches("/").to_string())
        } else {
            None
        };

    let id = id?;
    if id.is_empty() {
        return None;
    }

    Some(TYoutubeId { id, is_playlist })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_id_splitter() {
        assert_eq!(get_youtube_id("test-driven-development", false), None);
        assert_eq!(
            get_youtube_id("https://www.youtube.com/embed/ZNYRjxJ3sdY", false),
            Some(TYoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/shorts/ZNYRjxJ3sdY", false),
            Some(TYoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=ZNYRjxJ3sdY", false),
            Some(TYoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/playlist?list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS",
                false
            ),
            Some(TYoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/playlist?list=", false),
            None
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/playlist?list=", true),
            None
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=", false),
            None,
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=", true),
            None,
        );
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS",
                false
            ),
            Some(TYoutubeId {
                id: "rdXw7Ps9vxc".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=", false),
            Some(TYoutubeId {
                id: "rdXw7Ps9vxc".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/watch?v=e8T92lECvRA&list=PLKyFAP_ydM14amZrAVM39Jkfcy8NNCsv2&index=4",
                false
            ),
            Some(TYoutubeId {
                id: "e8T92lECvRA".to_string(),
                is_playlist: false,
            })
        );
        assert_eq!(
            get_youtube_id("https://youtu.be/sEWIDdQKWgc?si=Ywu5MycwAaZ4cZ3t", false),
            Some(TYoutubeId {
                id: "sEWIDdQKWgc".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://youtu.be/?si=Ywu5MycwAaZ4cZ3t", false),
            None,
        );
        assert_eq!(get_youtube_id("https://youtu.be/", false), None,);
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=&list=", false),
            None,
        )
    }

    // Auto-generated tests below.

    #[test]
    fn test_tyoutubeid_struct() {
        // Test basic construction and getters
        let vid = TYoutubeId::new("sEWIDdQKWgc", false);
        assert_eq!(vid.id(), "sEWIDdQKWgc");
        assert_eq!(vid.is_playlist(), false);

        let playlist = TYoutubeId::new("PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS", true);
        assert_eq!(playlist.id(), "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS");
        assert_eq!(playlist.is_playlist(), true);

        // Test Display implementation
        assert_eq!(format!("{}", vid), "sEWIDdQKWgc");
        assert_eq!(
            format!("{}", playlist),
            "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
        );
    }

    #[test]
    fn test_force_playlist_behavior() {
        // When force_playlist is true, it should prioritize extracting the 'list' ID over 'v'
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS",
                true
            ),
            Some(TYoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );

        // Even with force_playlist=true, if there is no list, it should return None
        // (because it wants a playlist but only finds a video)
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=rdXw7Ps9vxc", true),
            None
        );

        // If it's explicitly a watch url but only has a list (edge case)
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/watch?list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS",
                false
            ),
            Some(TYoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );
    }

    #[test]
    fn test_music_youtube_domain() {
        // Test video on YouTube Music
        assert_eq!(
            get_youtube_id("https://music.youtube.com/watch?v=ZNYRjxJ3sdY", false),
            Some(TYoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );

        // Test playlist on YouTube Music
        assert_eq!(
            get_youtube_id(
                "https://music.youtube.com/playlist?list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS",
                false
            ),
            Some(TYoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );
    }

    #[test]
    fn test_invalid_domains_and_malformed_urls() {
        // Valid URL structure, but wrong domain
        assert_eq!(
            get_youtube_id("https://www.vimeo.com/watch?v=ZNYRjxJ3sdY", false),
            None
        );

        assert_eq!(
            get_youtube_id("https://google.com/shorts/ZNYRjxJ3sdY", false),
            None
        );

        // Completely invalid URLs
        assert_eq!(get_youtube_id("not-a-url", false), None);
        assert_eq!(get_youtube_id("http://", false), None);
        assert_eq!(get_youtube_id("https://www.youtube.com/watch", false), None); // missing query
    }
}
