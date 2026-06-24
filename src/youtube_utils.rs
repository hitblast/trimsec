use reqwest::Url;

pub fn get_youtube_api_key() -> Option<String> {
    std::env::var("TRIMSEC_YOUTUBE_KEY").ok()
}

pub fn get_youtube_video_id(link: String) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(&link) {
        let id: Option<String> = if link.starts_with("https://www.youtube.com/embed/")
            || link.starts_with("https://www.youtube.com/shorts/")
        {
            parsed_url
                .path_segments()
                .and_then(|f| f.last())
                .map(|s| s.to_string())
        } else {
            parsed_url.query().and_then(|q| {
                q.split('&')
                    .find(|p| p.starts_with("v="))
                    .map(|p| p.trim_start_matches("v=").to_string())
            })
        };

        id
    } else {
        None
    }
}
