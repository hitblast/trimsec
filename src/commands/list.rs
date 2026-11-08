use crate::core::{
    api::types::YtClientCommon,
    context::Ctx,
    youtils::{TYoutubeId, TYoutubeVideo, get_youtube_id},
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
        let id: TYoutubeId = match get_youtube_id(&self.link, true) {
            Some(id) => {
                if !id.is_playlist() {
                    bail!("not a valid YouTube playlist ID!")
                }
                id
            }
            None => bail!("no YouTube playlist ID was found in this link."),
        };

        let videos: Vec<TYoutubeVideo> = ctx
            .client()?
            .fetch_video_items(&id, 0)
            .map_err(|e| anyhow::anyhow!("Failed to fetch playlist videos: {e}"))?;

        for video in &videos {
            println!("{}", video.title());
        }

        Ok(())
    }
}
