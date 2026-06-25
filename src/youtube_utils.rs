use reqwest::Url;

pub fn get_youtube_api_key() -> Option<String> {
    std::env::var("TRIMSEC_YOUTUBE_KEY").ok()
}

#[derive(Debug, PartialEq)]
pub struct YoutubeId {
    pub id: String,
    pub is_playlist: bool,
}

pub fn get_youtube_id(link: &str) -> Option<YoutubeId> {
    let mut is_playlist = false;

    if let Ok(parsed_url) = Url::parse(link) {
        if !parsed_url
            .host_str()
            .is_some_and(|f| f == "www.youtube.com" || f == "youtube.com")
        {
            return None;
        }

        let id: Option<String> = if parsed_url.path().starts_with("/shorts/")
            || parsed_url.path().starts_with("/embed/")
        {
            parsed_url
                .path_segments()
                .and_then(|f| f.last())
                .map(|s| s.to_string())
        } else if parsed_url.path().starts_with("/watch") {
            parsed_url.query().and_then(|q| {
                let pointer = if q.contains("list=") {
                    is_playlist = true;
                    "list="
                } else {
                    "v="
                };

                q.split('&')
                    .find(|p| p.starts_with(pointer))
                    .map(|p| p.trim_start_matches(pointer).to_string())
            })
        } else {
            return None;
        };

        if id == Some("".to_string()) {
            None
        } else if let Some(id) = id {
            Some(YoutubeId { id, is_playlist })
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
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
            None
        )
    }
}
