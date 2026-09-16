use clap::Args;

use crate::{
    commands::{Runnable, trim::TrimCmd},
    core::{
        api::ApiClientManager,
        style::Style,
        time::ToStringTime,
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};

#[derive(Debug, Default, Args)]
pub struct YtCmd {
    /// The URL, or link, for the YouTube video.
    link: String,

    /// The multiplier (e.g. 1.25x, 1.25).
    multiplier: String,

    /// Max amount of items to traverse in a playlist (if one is passed). Defaults to the total length of the playlist.
    #[arg(visible_alias = "max", long, default_value = "0")]
    max_items: usize,
}

impl Runnable for YtCmd {
    fn run(self, style: &Style) -> Result<()> {
        let key = get_youtube_api_key()?;

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&self.link);

        if let Some(id) = id {
            match manager.fetch_duration_from_id(&id, self.max_items) {
                Ok(dur) => {
                    let cmd = TrimCmd {
                        duration: dur.seconds().to_string_duration(),
                        multiplier: self.multiplier,
                    };

                    cmd.run(style)?;
                    if id.is_playlist() {
                        println!("Trimmed for {} item(s).", dur.splits())
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
