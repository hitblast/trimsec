use reqwest::Url;

pub fn get_youtube_api_key() -> Option<String> {
    std::env::var("TRIMSEC_YOUTUBE_KEY").ok()
}

pub fn get_youtube_video_id(link: &str) -> Option<String> {
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
                q.split('&')
                    .find(|p| p.starts_with("v="))
                    .map(|p| p.trim_start_matches("v=").to_string())
            })
        } else {
            return None;
        };

        if id == Some("".to_string()) { None } else { id }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_splitter() {
        assert_eq!(get_youtube_video_id("test-driven-development"), None);
        assert_eq!(
            get_youtube_video_id("https://www.youtube.com/embed/ZNYRjxJ3sdY"),
            Some("ZNYRjxJ3sdY".to_string())
        );
        assert_eq!(
            get_youtube_video_id("https://www.youtube.com/shorts/ZNYRjxJ3sdY"),
            Some("ZNYRjxJ3sdY".to_string())
        );
        assert_eq!(
            get_youtube_video_id("https://www.youtube.com/watch?v=ZNYRjxJ3sdY"),
            Some("ZNYRjxJ3sdY".to_string())
        );
        assert_eq!(
            get_youtube_video_id("https://www.youtube.com/watch?v="),
            None,
        );
    }
}
