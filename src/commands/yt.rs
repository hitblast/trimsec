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
}

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
            if !id.is_playlist {
                match manager.fetch_duration_from_id(&id.id) {
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
                bail!("the code is there, but playlists aren't supported *yet*")
            }
        } else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `yt` command."
            )
        }

        Ok(())
    }
}
