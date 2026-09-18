use crate::{
    args::CmdContentType,
    commands::Ctx,
    core::{
        api::ApiClient,
        time::{TDuration, ToStringTime, time_in_day_after},
        youtils::{decide_youtube_key, get_youtube_id},
    },
};
use anyhow::{Result, bail};

pub struct FitCmd {
    pub content: CmdContentType,
    pub budget: Option<TDuration>,
    pub max_items: usize,
}

impl FitCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        let key = decide_youtube_key(ctx.config()?)?;

        let manager = ApiClient::new(&key);

        let id = get_youtube_id(&self.link);

        let Some(id) = id else {
            bail!(
                "Not a valid YouTube URL! Only videos/embeds/shorts URLs are supported in the `fit` command."
            )
        };

        let yt_dur = manager
            .fetch_duration_from_id(&id, self.max_items)
            .map_err(|e| anyhow::anyhow!("Failed to fetch details from URL: {e}"))?;

        let message = {
            let status = if let Some(budget_dur) = &self.budget {
                if budget_dur > &yt_dur {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        budget_dur - &yt_dur
                    )
                } else if budget_dur < &yt_dur {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        &yt_dur - &budget_dur,
                        ctx.style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            } else {
                let time_left = time_in_day_after(yt_dur.seconds());

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

            format!("\n{status}\n(counted {} videos)\n", yt_dur.splits())
        };

        println!("{message}");
        Ok(())
    }
}
