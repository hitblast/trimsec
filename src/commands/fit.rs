use crate::{
    args::TCmdContent,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime, time_in_day_after},
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
        let content_dur = match self.content {
            TCmdContent::Raw(tduration) => tduration,
            TCmdContent::YouTube(youtube_id) => {
                let max = ctx.kwargs.max_items();
                ctx.client()?
                    .fetch_duration_from_id(&youtube_id, max)
                    .map_err(|e| anyhow::anyhow!("Failed to fetch details from URL: {e}"))?
            }
        };

        let cfg_budget = ctx
            .config()?
            .options()
            .and_then(|f| f.default_fit_budget())
            .cloned();

        let budget = if self.budget.is_some() {
            self.budget.as_ref()
        } else if cfg_budget.is_some() {
            cfg_budget.as_ref()
        } else {
            None
        };

        let message = {
            let status = if let Some(budget) = budget {
                if budget > &content_dur {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        budget - &content_dur
                    )
                } else if budget < &content_dur {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        &content_dur - budget,
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
