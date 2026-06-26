use arboard::Clipboard;
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
    #[arg(short, long)]
    link: Option<String>,

    /// The multiplier (e.g. 1.25x, 1.25).
    #[arg(short, long)]
    multiplier: String,

    /// Fetch link from the clipboard.
    #[arg(short, long)]
    clip: bool,

    /// Max items to traverse (applicable for playlists). Defaults to: 500.
    #[arg(long)]
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

        if self.link.is_some() && self.clip {
            bail!("-l and -c cannot be used at the same time!")
        }

        let link = if self.clip {
            let mut c = Clipboard::new().unwrap();
            let l = c.get_text().ok();

            if let Some(l) = l {
                l
            } else {
                bail!("No content found in clipboard.")
            }
        } else if let Some(l) = self.link {
            l
        } else {
            bail!("Could not find YouTube link in arguments. Aborting.")
        };

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&link);

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
