use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YTCrudeResponse {
    items: Vec<YTCrudeResponseItem>,
}

#[derive(Debug, Deserialize)]
struct YTCrudeResponseItem {
    #[serde(rename = "contentDetails")]
    content_details: YTCrudeResponseContentDetails,
}

#[derive(Debug, Deserialize)]
struct YTCrudeResponseContentDetails {
    duration: String,
}

pub struct ApiClientManager<'a> {
    client: Client,
    key: &'a str,
}

impl<'a> ApiClientManager<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            client: Client::new(),
            key: key,
        }
    }

    pub fn fetch_duration_from_id(&self, id: &str) -> Result<String> {
        // sample video link: https://www.youtube.com/watch?v=D4iiKkjGJmU
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={id}&key={}&part=contentDetails",
            self.key
        );
        let response: YTCrudeResponse = self.client.get(url).send()?.json()?;

        Ok(response
            .items
            .first()
            .unwrap()
            .content_details
            .duration
            .to_owned()
            .to_lowercase()
            .trim_start_matches("pt")
            .to_string())
    }
}
