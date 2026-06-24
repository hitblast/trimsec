use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

pub fn get_youtube_api_key() -> Option<String> {
    std::env::var("TRIMSEC_YOUTUBE_KEY").ok()
}

#[derive(Debug, Deserialize)]
pub struct YTResponseDetails {
    pub duration: String,
}

pub struct ApiClientManager {
    client: Client,
    key: String,
}

impl ApiClientManager {
    pub fn new(key: &str) -> Self {
        Self {
            client: Client::new(),
            key: key.to_owned(),
        }
    }

    pub fn get_details_for_id(&self, id: &str) -> Result<YTResponseDetails> {
        // sample video link: https://www.youtube.com/watch?v=D4iiKkjGJmU
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={id}&key={}&part=contentDetails",
            self.key
        );
        let response: YTResponseDetails = self.client.get(url).send()?.json()?;
        Ok(response)
    }
}
