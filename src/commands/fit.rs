use crate::{
    commands::{Ctx, Runnable},
    core::{
        api::ApiClient,
        time::{TDuration, ToStringTime, time_in_day_after},
        youtils::{decide_youtube_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct FitCmd {
    /// The URL, or link, for the YouTube video.
    link: String,

    /// The budget duration string. By default uses the remaining time for the day.
    budget: Option<String>,

    /// Max amount of items to traverse in a playlist.
    #[arg(short, long, visible_alias = "max", default_value = "0")]
    max_items: usize,
}

impl Runnable for FitCmd {
    fn run(self, ctx: &mut Ctx) -> Result<()> {
        let key = decide_youtube_key(ctx.config()?)?;

        let manager = ApiClient::new(&key);
        let id = get_youtube_id(&self.link);

        let Some(id) = id else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `fit` command."
            )
        };

        let vid_total_duration = manager
            .fetch_duration_from_id(&id, self.max_items)
            .map_err(|e| anyhow::anyhow!("Failed to fetch details from URL: {e}"))?;

        let message = {
            let status = if let Some(b) = &self.budget {
                let limit_duration = TDuration::parse_str(b)
                    .map_err(|e| anyhow::anyhow!("Failed to parse budget duration: {e}"))?;

                if limit_duration > vid_total_duration {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        &limit_duration - &vid_total_duration
                    )
                } else if limit_duration < vid_total_duration {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        &vid_total_duration - &limit_duration,
                        ctx.style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            } else {
                let time_left = time_in_day_after(vid_total_duration.seconds());

                if time_left != 0.0 {
                    format!(
                        "{}Fits in day!{}\n\nTime left afterwards: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        time_left.to_string_duration()
                    )
                } else {
                    format!(
                        "{}Content does not fit in the day.{}",
                        ctx.style.boldred(),
                        ctx.style.reset()
                    )
                }
            };

            format!(
                "\n{status}\n(counted {} videos)\n",
                vid_total_duration.splits()
            )
        };

        println!("{message}");
        Ok(())
    }
}
