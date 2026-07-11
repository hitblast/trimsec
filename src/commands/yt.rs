use clap::Args;

use crate::{
    commands::trim::TrimCmd,
    core::{
        api::ApiClientManager,
        time::parse_time,
        utils::choose_or_grab_link,
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
}

impl YtCmd {
    pub fn run(self, no_clip: bool) -> Result<()> {
        let key = get_youtube_api_key()?;
        let link = choose_or_grab_link(self.link, no_clip)?;

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
