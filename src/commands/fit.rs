use crate::{
    args::TCmdContent,
    commands::Ctx,
    core::{
        api::ApiClient,
        time::{TDuration, ToStringTime, time_in_day_after},
        youtils::decide_youtube_key,
    },
};
use anyhow::Result;

pub struct FitCmd {
    content: TCmdContent,
    budget: Option<TDuration>,
}

impl FitCmd {
    #[must_use]
    pub fn new(content: TCmdContent, budget: Option<TDuration>) -> Self {
        Self { content, budget }
    }

    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        let key = decide_youtube_key(ctx.config()?)?;

        let content_dur = match self.content {
            TCmdContent::Raw(tduration) => tduration,
            TCmdContent::YouTube(youtube_id) => {
                let manager = ApiClient::new(&key);

                manager
                    .fetch_duration_from_id(&youtube_id, ctx.kwargs.max_items())
                    .map_err(|e| anyhow::anyhow!("Failed to fetch details from URL: {e}"))?
            }
        };

        let message = {
            let status = if let Some(budget_dur) = &self.budget {
                if budget_dur > &content_dur {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        budget_dur - &content_dur
                    )
                } else if budget_dur < &content_dur {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        &content_dur - budget_dur,
                        ctx.style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            } else {
                let time_left = time_in_day_after(content_dur.seconds());

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

            format!("\n{status}\n(counted {} splits)\n", content_dur.splits())
        };

        println!("{message}");
        Ok(())
    }
}
