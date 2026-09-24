use crate::{
    args::{TCmdContent, Token},
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime, time_in_day_after},
        youtils::TYoutubeId,
    },
};
use anyhow::{Result, bail};

pub struct FitCmd {
    content: TCmdContent,
    determined_budget: Option<TDuration>,
}

impl FitCmd {
    fn new(content: TCmdContent, determined_budget: Option<TDuration>) -> Self {
        Self {
            content,
            determined_budget,
        }
    }

    pub fn delegate_tokens(tokens: Vec<Token>) -> Result<Self> {
        let mut budget_duration: Option<TDuration> = None;
        let mut yt_ids: Vec<TYoutubeId> = Vec::new();

        let bare_durations: Vec<TDuration> = tokens
            .iter()
            .filter_map(|f| match f {
                Token::Duration(dur) => Some(dur.clone()),
                Token::YouTube(id) => {
                    yt_ids.push(id.clone());
                    None
                }
                Token::BudgetDuration(dur) => {
                    match &mut budget_duration {
                        Some(existing) => *existing += dur,
                        None => budget_duration = Some(dur.clone()),
                    }
                    None
                }
                _ => None,
            })
            .collect();

        if bare_durations.is_empty() {
            bail!("Missing content duration for fit-check.")
        }

        let cmd: Self = if bare_durations.len() + 1 == tokens.len() {
            if bare_durations.len() == 2 {
                Self::new(
                    TCmdContent::Raw(bare_durations[0].clone()),
                    Some(bare_durations[1].clone()),
                )
            } else {
                let sum = bare_durations.into_iter().sum();
                Self::new(TCmdContent::Raw(sum), budget_duration)
            }
        } else {
            Self::new(
                TCmdContent::Complex((bare_durations, yt_ids)),
                budget_duration,
            )
        };

        Ok(cmd)
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
            TCmdContent::Complex((mut durations, yt_ids)) => {
                let max = ctx.kwargs.max_items();
                let client = ctx.client()?;

                let total_yt_duration: TDuration = yt_ids
                    .iter()
                    .filter_map(|id| client.fetch_duration_from_id(id, max).ok())
                    .sum::<TDuration>();

                durations.push(total_yt_duration);
                durations.into_iter().sum()
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
