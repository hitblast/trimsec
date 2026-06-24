use reqwest::blocking::Client;
use serde::Deserialize;

use crate::errors::TYoutubeError;

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

    pub fn fetch_duration_from_id(self, id: &str) -> Result<String, TYoutubeError> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={id}&key={}&part=contentDetails",
            self.key
        );
        let response: YTCrudeResponse = self
            .client
            .get(url)
            .send()
            .map_err(|e| TYoutubeError::Reqwest(e))?
            .json()
            .map_err(|e| TYoutubeError::Reqwest(e))?;

        if let Some(item) = response.items.first() {
            return Ok(item
                .content_details
                .duration
                .trim_start_matches("PT")
                .to_lowercase());
        } else {
            Err(TYoutubeError::ItemNotFound)
        }
    }
}
