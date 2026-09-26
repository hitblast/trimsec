use crate::{
    args::Token::{self},
    core::{context::Ctx, time::TDuration, timeutils::time_in_day_left},
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

        let tokens_len = tokens.len();
        let mut durations: Vec<TDuration> = Vec::new();

        let mut iterable = tokens.into_iter();
        while let Some(tok) = iterable.next() {
            match tok {
                Token::Duration(dur) => durations.push(dur),
                Token::BudgetDuration(dur) => match &mut budget_duration {
                    Some(existing) => *existing += &dur,
                    None => budget_duration = Some(dur),
                },
                Token::YouTube((id, max_items)) => match ctx.client() {
                    Ok(client) => match client.fetch_duration_from_id(&id, max_items) {
                        Ok(dur) => durations.push(dur),
                        Err(e) => bail!("failed to fetch duration: {e}"),
                    },
                    Err(e) => bail!("client failed: {e}"),
                },
                _ => {}
            }
        }

        if durations.is_empty() {
            bail!("missing content duration for fit-check.")
        }

        let cmd: FitCmd<'_> = if durations.len() + 1 == tokens_len && durations.len() == 2 {
            FitCmd::new(ctx, durations[0].clone(), Some(durations[1].clone()))
        } else {
            let sum = durations.into_iter().sum();
            FitCmd::new(ctx, sum, budget_duration)
        };

        Ok(cmd)
    }

    pub fn run(self) -> Result<()> {
        let content_duration: TDuration = self.duration;
        let ctx: &'a mut Ctx = self.ctx;

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

        let message: String = {
            let header: String = format!(
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
