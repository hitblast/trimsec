use anyhow::{Result as YoutilsResult, bail};
use reqwest::Url;

use crate::core::config::Config;

#[must_use]
pub fn get_youtube_api_key() -> YoutilsResult<String> {
    const ENV_VAR_NAME: &str = "TRIMSEC_YOUTUBE_KEY";
    let x = std::env::var(ENV_VAR_NAME).ok();

    match x {
        Some(x) => Ok(x),
        None => {
            let cfg_res = Config::load();

            match cfg_res {
                Ok(cfg) => Ok(cfg.api_key().to_string()),
                Err(e) => {
                    match e {
                        crate::errors::TConfigError::ParseFailed(p) => {
                            bail!("Failed to parse .trimsecrc file at path: {p:?}")
                        }
                        _ => {}
                    }
                    bail!(
                        "Missing {ENV_VAR_NAME} environment variable or .trimsecrc file in $HOME; read README.md to learn more."
                    )
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct YoutubeId {
    pub id: String,
    pub is_playlist: bool,
}

#[must_use]
pub fn get_youtube_id(link: &str) -> Option<YoutubeId> {
    let mut is_playlist = false;

    if let Ok(parsed_url) = Url::parse(link) {
        if !parsed_url
            .host_str()
            .is_some_and(|f| f == "www.youtube.com" || f == "youtube.com" || f == "youtu.be")
        {
            return None;
        }

        let query_search = |q: &str, ptr: &str| {
            q.split('&')
                .find(|p| p.starts_with(ptr))
                .map(|p| p.trim_start_matches(ptr).to_string())
        };

        let id: Option<String> = if parsed_url.path().starts_with("/shorts/")
            || parsed_url.path().starts_with("/embed/")
        {
            parsed_url
                .path_segments()
                .and_then(|mut f| f.next_back())
                .map(|s| s.to_string())
        } else if parsed_url.path().starts_with("/watch") {
            parsed_url.query().and_then(|q| {
                if q.contains("list=") {
                    let listsearch = query_search(q, "list=");

                    if listsearch.is_none() || listsearch.clone().is_some_and(|f| f.is_empty()) {
                        query_search(q, "v=")
                    } else {
                        is_playlist = true;
                        listsearch
                    }
                } else {
                    query_search(q, "v=")
                }
            })
        } else if parsed_url.path().starts_with("/playlist") {
            is_playlist = true;
            parsed_url.query().and_then(|q| query_search(q, "list="))
        } else if parsed_url.host_str().is_some_and(|f| f == "youtu.be") {
            Some(parsed_url.path().trim_start_matches("/").to_string())
        } else {
            return None;
        };

        if id == Some("".to_string()) {
            None
        } else {
            id.map(|id| YoutubeId { id, is_playlist })
        }
    } else {
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_id_splitter() {
        assert_eq!(get_youtube_id("test-driven-development"), None);
        assert_eq!(
            get_youtube_id("https://www.youtube.com/embed/ZNYRjxJ3sdY"),
            Some(YoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/shorts/ZNYRjxJ3sdY"),
            Some(YoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=ZNYRjxJ3sdY"),
            Some(YoutubeId {
                id: "ZNYRjxJ3sdY".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/playlist?list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
            ),
            Some(YoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/playlist?list="),
            None
        );
        assert_eq!(get_youtube_id("https://www.youtube.com/watch?v="), None,);
        assert_eq!(
            get_youtube_id(
                "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
            ),
            Some(YoutubeId {
                id: "PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS".to_string(),
                is_playlist: true
            })
        );
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=rdXw7Ps9vxc&list="),
            Some(YoutubeId {
                id: "rdXw7Ps9vxc".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://youtu.be/sEWIDdQKWgc?si=Ywu5MycwAaZ4cZ3t"),
            Some(YoutubeId {
                id: "sEWIDdQKWgc".to_string(),
                is_playlist: false
            })
        );
        assert_eq!(
            get_youtube_id("https://youtu.be/?si=Ywu5MycwAaZ4cZ3t"),
            None,
        );
        assert_eq!(get_youtube_id("https://youtu.be/"), None,);
        assert_eq!(
            get_youtube_id("https://www.youtube.com/watch?v=&list="),
            None,
        )
    }
}
