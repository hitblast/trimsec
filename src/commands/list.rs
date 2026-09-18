use crate::{
    commands::Ctx,
    core::{
        api::ApiClient,
        youtils::{decide_youtube_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct ListCmd {
    /// The link to the YouTube playlist.
    link: String,

    /// The maximum amount of items to list from the given playlist.
    #[arg(long, default_value = "0")]
    max_items: usize,
}

impl ListCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        let key = decide_youtube_key(ctx.config()?)?;

        let manager = ApiClient::new(&key);
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
