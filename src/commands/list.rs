use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{
        api::ApiClientManager,
        style::Style,
        utils::choose_or_grab_link,
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct ListCmd {
    /// The link to the YouTube playlist.
    pub link: Option<String>,
}

impl Runnable for ListCmd {
    fn run(self, flags: &Flags, _: &Style) -> Result<()> {
        let link = choose_or_grab_link(self.link, flags.no_clip)?;
        let key = get_youtube_api_key()?;

        let manager = ApiClientManager::new(&key);
        let id = match get_youtube_id(&link) {
            Some(id) => {
                if !id.is_playlist {
                    bail!("Not a valid YouTube playlist ID!")
                }
                id
            }
            None => bail!("No YouTube playlist ID was found in this link."),
        };

        let ids = manager
            .fetch_ids_from_id(&id, 0)
            .map_err(|e| anyhow::anyhow!("Failed to get playlist item IDs: {e}"))?;
        let videos = manager
            .fetch_video_items(&ids)
            .map_err(|e| anyhow::anyhow!("Failed to fetch playlist videos: {e}"))?;

        for v in videos {
            println!("{}", v.snippet.title)
        }

        Ok(())
    }
}
