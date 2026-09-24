use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime},
    },
};
use anyhow::{Result, bail};

pub struct TrimCmd<'a> {
    ctx: &'a mut Ctx,
    duration: TDuration,
    multiplier: f64,
}

impl<'a> TrimCmd<'a> {
    fn new(ctx: &'a mut Ctx, duration: TDuration, multiplier: f64) -> Self {
        Self {
            ctx,
            duration,
            multiplier,
        }
    }

    pub fn delegate(ctx: &'a mut Ctx, tokens: Vec<Token>) -> Result<Vec<Self>> {
        let cur_dur: Option<TDuration> = None;
        let cur_mul: Option<f64> = None;
        let runnables: Vec<Self> = Vec::new();

        while let Some(tok) = tokens.iter().next() {}

        Ok(runnables)
    }

    pub fn run(&mut self) -> Result<()> {
        if self.multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
        }

        if self.ctx.kwargs.max_items() != 0 {
            bail!("--max-items cannot be used for regular durations.")
        }

        let dur = &mut self.duration;
        dur.trim(self.multiplier);

        let remaining = crate::core::time::time_in_day_after(dur.seconds());
        let saved = dur.saved_time().to_string_duration();

        let message = [
            format!(
                "\nFinishes in: {} ",
                if dur.splits() > 1 {
                    format!("{dur}")
                } else {
                    dur.to_string()
                }
            ),
            if remaining != 0.0 {
                format!("Time in day left: {} ", remaining.to_string_duration())
            } else {
                "Cannot finish today.".to_string()
            },
            format!(
                "{}Saved {saved}!{}\n\nTrimmed for {} item(s).",
                self.ctx.style.boldgreen(),
                self.ctx.style.reset(),
                dur.splits()
            ),
        ]
        .join("\n");

        println!("{message}");

        Ok(())
    }
}
