use clap::Args;

use crate::{
    commands::{Runnable, trim::TrimCmd},
    core::api::ApiClientManager,
    youtube_utils::{get_youtube_api_key, get_youtube_id},
};
use anyhow::{Result, bail};

#[derive(Debug, Default, Args)]
pub struct YtCmd {
    /// The URL, or link, for the YouTube video.
    link: String,

    /// The multiplier (e.g. 1.25x, 1.25).
    multiplier: String,

    /// Max items to traverse (applicable for playlists). Defaults to: 500.
    #[arg(short, long)]
    max_items: Option<usize>,
}

const YT_PLAYLIST_MAX_ITEMS: usize = 500;

impl Runnable for YtCmd {
    fn run(self) -> Result<()> {
        let key = match get_youtube_api_key() {
            Some(key) => key,
            None => bail!(
                "Missing environment variable: TRIMSEC_YOUTUBE_KEY; read README.md for more information."
            ),
        };

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&self.link);

        if let Some(id) = id {
            match manager
                .fetch_duration_from_id(id, self.max_items.unwrap_or(YT_PLAYLIST_MAX_ITEMS))
            {
                Ok(duration) => {
                    let cmd = TrimCmd {
                        duration,
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
