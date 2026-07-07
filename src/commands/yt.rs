use arboard::Clipboard;
use clap::Args;

use crate::{
    commands::trim::TrimCmd,
    core::{
        api::ApiClientManager,
        time::parse_time,
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};

#[derive(Debug, Default, Args)]
pub struct YtCmd {
    /// The URL, or link, for the YouTube video.
    #[arg(short, long)]
    link: Option<String>,

    /// The multiplier (e.g. 1.25x, 1.25).
    #[arg(short, long)]
    multiplier: String,

    /// Max amount of items to traverse in a playlist. Default: 0 (uncapped).
    #[arg(long, default_value = "0")]
    max_items: usize,

    /// Disable grabbing links from clipboard.
    #[arg(short, long)]
    noclip: bool,
}

impl YtCmd {
    pub fn run(self) -> Result<()> {
        let key = match get_youtube_api_key() {
            Some(key) => key,
            None => bail!(
                "Missing environment variable: TRIMSEC_YOUTUBE_KEY; read README.md for more information."
            ),
        };

        let link = if self.link.is_none() && !self.noclip {
            let mut c = Clipboard::new()?;
            let l = c.get_text().ok();

            if let Some(l) = l {
                l
            } else {
                bail!("No content found in clipboard.")
            }
        } else if let Some(l) = self.link {
            l
        } else {
            bail!("Link to YouTube object (video/playlist) is required. Aborting.")
        };

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&link);

        if let Some(id) = id {
            match manager.fetch_duration_from_id(&id, self.max_items) {
                Ok((duration, item_count)) => {
                    let cmd = TrimCmd {
                        duration: parse_time(duration),
                        multiplier: self.multiplier,
                    };

                    cmd.run()?;
                    if id.is_playlist {
                        println!("Trimmed for {item_count} item(s).")
                    }
                }
                Err(e) => bail!("Failed to fetch details from URL: {e}"),
            }
        } else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `yt` command."
            )
        }

        Ok(())
    }
}
