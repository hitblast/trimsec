use clap::Args;
use reqwest::Url;

use crate::{
    api::ApiClientManager,
    commands::{Runnable, trim::TrimCmd},
};
use anyhow::{Result, bail};

#[derive(Debug, Default, Args)]
pub struct YtCmd {
    /// The URL, or link, for the YouTube video.
    link: String,

    /// The multiplier (e.g. 1.25x, 1.25).
    multiplier: String,
}

pub const TRIMSEC_YOUTUBE_KEY: &'static str = "TRIMSEC_YOUTUBE_KEY";

impl Runnable for YtCmd {
    fn run(self) -> Result<()> {
        let key = match std::env::var(TRIMSEC_YOUTUBE_KEY) {
            Ok(key) => key,
            Err(_) => bail!(
                "Missing environment variable: TRIMSEC_YOUTUBE_KEY; read README.md for more information."
            ),
        };

        let manager = ApiClientManager::new(&key);

        let parsed_url = Url::parse(&self.link)?;
        let parsed_url_str = parsed_url.to_string();

        let id: Option<String> = if parsed_url_str.starts_with("https://www.youtube.com/embed/")
            || parsed_url_str.starts_with("https://www.youtube.com/shorts/")
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

        if let Some(id) = id {
            match manager.get_details_for_id(&id) {
                Ok(details) => {
                    let cmd = TrimCmd {
                        duration: details.duration,
                        multiplier: self.multiplier,
                    };

                    cmd.run()?
                }
                Err(e) => bail!("failed to fetch details from URL: {e}"),
            }
        } else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `yt` command."
            )
        }

        Ok(())
    }
}
