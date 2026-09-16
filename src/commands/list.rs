use crate::{
    commands::Runnable,
    core::{
        api::ApiClientManager,
        style::Style,
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct ListCmd {
    /// The link to the YouTube playlist.
    link: String,

    /// The maximum amount of items to list from the given playlist.
    #[arg(short, long, visible_alias = "max", default_value = "0")]
    max_items: usize,
}

impl Runnable for ListCmd {
    fn run(self, _: &Style) -> Result<()> {
        let key = get_youtube_api_key()?;

        let manager = ApiClientManager::new(&key);
        let id = match get_youtube_id(&self.link) {
            Some(id) => {
                if !id.is_playlist() {
                    bail!("Not a valid YouTube playlist ID!")
                }
                id
            }
            None => bail!("No YouTube playlist ID was found in this link."),
        };

        let ids = manager
            .expand_id(&id, self.max_items)
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
