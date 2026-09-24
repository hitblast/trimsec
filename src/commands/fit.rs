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
    determined_budget: Option<TDuration>,
}

impl FitCmd {
    #[must_use]
    pub fn new(content: TCmdContent, determined_budget: Option<TDuration>) -> Self {
        Self {
            content,
            determined_budget,
        }
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
            TCmdContent::TokenVec(tokens) => {
                let max = ctx.kwargs.max_items();
                let client = ctx.client()?;

                let total_duration: TDuration = tokens
                    .into_iter()
                    .filter_map(|f| match f {
                        crate::args::Token::Duration(dur) => Some(dur),
                        crate::args::Token::YouTube(id) => {
                            client.fetch_duration_from_id(&id, max).ok()
                        }
                        _ => None,
                    })
                    .sum::<TDuration>();

                total_duration
            }
        };

        let cfg_budget = ctx
            .config()?
            .options()
            .and_then(|f| f.default_fit_budget())
            .cloned();

        let budget = if self.determined_budget.is_some() {
            self.determined_budget.as_ref()
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

            format!(
                "\n{status}\n({content_dur}; counted {} splits)\n",
                content_dur.splits()
            )
        };

        println!("{message}");
        Ok(())
    }
}
