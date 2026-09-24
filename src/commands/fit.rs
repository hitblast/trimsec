use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime, time_in_day_after},
    },
};
use anyhow::{Result, bail};

pub struct FitCmd<'a> {
    ctx: &'a mut Ctx,
    duration: TDuration,
    determined_budget: Option<TDuration>,
}

impl<'a> FitCmd<'a> {
    fn new(ctx: &'a mut Ctx, duration: TDuration, determined_budget: Option<TDuration>) -> Self {
        Self {
            ctx,
            duration,
            determined_budget,
        }
    }

    pub fn delegate(ctx: &'a mut Ctx, tokens: Vec<Token>) -> Result<Self> {
        let mut budget_duration: Option<TDuration> = None;
        let max = ctx.kwargs.max_items();

        let durations: Vec<TDuration> = tokens
            .iter()
            .filter_map(|f| match f {
                Token::Duration(dur) => Some(dur.clone()),
                Token::YouTube(id) => ctx
                    .client()
                    .ok()
                    .and_then(|f| f.fetch_duration_from_id(id, max).ok()),
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

        if durations.is_empty() {
            bail!("Missing content duration for fit-check.")
        }

        let cmd = if durations.len() + 1 == tokens.len() && durations.len() == 2 {
            FitCmd::new(ctx, durations[0].clone(), Some(durations[1].clone()))
        } else {
            let sum = durations.into_iter().sum();
            FitCmd::new(ctx, sum, budget_duration)
        };

        Ok(cmd)
    }

    pub fn run(self) -> Result<()> {
        let content_duration = self.duration;
        let ctx = self.ctx;

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
                if budget > &content_duration {
                    format!(
                        "{}Fits in budget!{}\n\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        budget - &content_duration
                    )
                } else if budget < &content_duration {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        &content_duration - budget,
                        ctx.style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            } else {
                let time_left = time_in_day_after(content_duration.seconds());

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
                "\n{status}\n({content_duration}; counted {} splits)\n",
                content_duration.splits()
            )
        };

        println!("{message}");
        Ok(())
    }
}
