use crate::core::{
    context::Ctx,
    youtils::{TYoutubeId, get_youtube_id},
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct ListCmd {
    /// The link to the YouTube playlist.
    link: String,
}

impl ListCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        let id: TYoutubeId = match get_youtube_id(&self.link) {
            Some(id) => {
                if !id.is_playlist() {
                    bail!("not a valid YouTube playlist ID!")
                }
                id
            }
            None => bail!("no YouTube playlist ID was found in this link."),
        };

        let ids: Vec<String> = ctx
            .client()?
            .expand_id(&id, 0)
            .map_err(|e| anyhow::anyhow!("Failed to get playlist item IDs: {e}"))?;
        let videos = ctx
            .client()?
            .fetch_video_items(&ids)
            .map_err(|e| anyhow::anyhow!("Failed to fetch playlist videos: {e}"))?;

        for v in videos {
            println!("{}", v.snippet.title)
        }

        Ok(())
    }
}
