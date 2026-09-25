use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, time_in_day_left},
    },
};
use anyhow::{Result, bail};

pub struct FitCmd<'a> {
    ctx: &'a mut Ctx,
    duration: TDuration,
    budget: Option<TDuration>,
}

impl<'a> FitCmd<'a> {
    fn new(ctx: &'a mut Ctx, duration: TDuration, budget: Option<TDuration>) -> Self {
        Self {
            ctx,
            duration,
            budget,
        }
    }

    pub fn delegate(ctx: &'a mut Ctx, tokens: Vec<Token>) -> Result<Self> {
        let mut budget_duration: Option<TDuration> = None;
        let max = ctx.kwargs.max_items();

        let tokens_len = tokens.len();
        let durations: Vec<TDuration> = tokens
            .into_iter()
            .filter_map(|f| match f {
                Token::Duration(dur) => Some(dur.clone()),
                Token::YouTube(id) => ctx
                    .client()
                    .ok()
                    .and_then(|f| f.fetch_duration_from_id(&id, max).ok()),
                Token::BudgetDuration(dur) => {
                    match &mut budget_duration {
                        Some(existing) => *existing += &dur,
                        None => budget_duration = Some(dur),
                    }
                    None
                }
                _ => None,
            })
            .collect();

        if durations.is_empty() {
            bail!("Missing content duration for fit-check.")
        }

        let cmd = if durations.len() + 1 == tokens_len && durations.len() == 2 {
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

        let mut is_day_budget = false;
        let budget = if let Some(bud) = self.budget {
            bud
        } else if let Some(bud) = cfg_budget {
            bud
        } else {
            is_day_budget = true;
            time_in_day_left()
        };

        let message = {
            let header = format!(
                "{}{} / {}{}",
                ctx.style.grey(),
                content_duration,
                budget,
                ctx.style.reset()
            );

            let status = {
                if budget > content_duration {
                    format!(
                        "{}Fits in {}!{}\nExtra time left: {}",
                        ctx.style.boldgreen(),
                        if is_day_budget { "day" } else { "budget" },
                        ctx.style.reset(),
                        budget - content_duration
                    )
                } else if budget < content_duration {
                    format!(
                        "{}Time overrun by {}!{}",
                        ctx.style.boldred(),
                        content_duration - budget,
                        ctx.style.reset()
                    )
                } else {
                    "Duration match! Would finish on time.".to_string()
                }
            };
            format!("\n{header}\n\n{status}\n")
        };

        println!("{message}");
        Ok(())
    }
}
