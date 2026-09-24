use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime},
    },
};
use anyhow::{Result, bail};

pub struct TrimCmd {
    duration: TDuration,
    multiplier: f64,
}

impl TrimCmd {
    pub fn delegate(ctx: &mut Ctx, tokens: Vec<Token>) -> Result<Vec<Self>> {
        let mut cursor_duration: Option<TDuration> = None;
        let mut cursor_multiplier: Option<f64> = None;
        let mut runnables: Vec<Self> = Vec::new();

        let mut iterable = tokens.iter().peekable();
        while let Some(tok) = iterable.next() {
            match tok {
                Token::Duration(dur) => match &mut cursor_duration {
                    Some(total) => *total += dur,
                    None => cursor_duration = Some(dur.clone()),
                },
                Token::BudgetDuration(_) => bail!(
                    "Budget duration cannot be present inside an expression which prioritizes multipliers."
                ),
                Token::Multiplier(new) => match &mut cursor_multiplier {
                    Some(existing) => {
                        if let Some(duration) = cursor_duration.take() {
                            runnables.push(Self {
                                duration,
                                multiplier: *existing,
                            });
                            cursor_multiplier = Some(*new);
                        } else {
                            match iterable.peek() {
                                Some(Token::Duration(_)) | Some(Token::YouTube(_)) => {
                                    cursor_multiplier = Some(*new)
                                }
                                _ => {
                                    bail!(
                                        "Multiplier \"{existing}x, {new}x\" given but duration does not exist."
                                    )
                                }
                            }
                        }
                    }
                    None => match cursor_duration.take() {
                        Some(duration) => {
                            runnables.push(Self {
                                duration,
                                multiplier: *new,
                            });
                        }
                        None => cursor_multiplier = Some(*new),
                    },
                },
                Token::YouTube(id) => {
                    let max = ctx.kwargs.max_items();
                    let dur = ctx
                        .client()
                        .ok()
                        .and_then(|f| f.fetch_duration_from_id(id, max).ok());

                    if let Some(dur) = dur {
                        match &mut cursor_duration {
                            Some(total) => *total += &dur,
                            None => cursor_duration = Some(dur.clone()),
                        }
                    }
                }
                Token::EOL => {
                    if let Some(multiplier) = cursor_multiplier {
                        if let Some(duration) = cursor_duration.take() {
                            runnables.push(Self {
                                duration,
                                multiplier,
                            });
                        } else {
                            bail!("Unused multiplier: {multiplier}x")
                        }
                    } else if let Some(definitely_unused) = cursor_duration {
                        bail!("Unused duration: {definitely_unused}")
                    }
                }
            }
        }

        Ok(runnables)
    }

    pub fn run(&mut self, ctx: &mut Ctx) -> Result<()> {
        if self.multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
        }

        if ctx.kwargs.max_items() != 0 {
            bail!("--max-items cannot be used for regular durations.")
        }

        println!(
            "\n{}{} -> {}x{}",
            ctx.style.grey(),
            self.duration,
            self.multiplier,
            ctx.style.reset()
        );
        let dur = &mut self.duration;
        dur.trim(self.multiplier);

        let remaining = crate::core::time::time_in_day_after(dur.seconds());
        let saved = dur.saved_time().to_string_duration();

        let message = [
            format!(
                "\n{}Finishes in: {dur}{} {}({} items){}",
                ctx.style.bold(),
                ctx.style.reset(),
                ctx.style.grey(),
                dur.splits(),
                ctx.style.reset(),
            ),
            if remaining != 0.0 {
                format!("Time in day left: {} ", remaining.to_string_duration())
            } else {
                "Cannot finish today.".to_string()
            },
            format!(
                "{}Saved {saved}!{}\n",
                ctx.style.boldgreen(),
                ctx.style.reset(),
            ),
        ]
        .join("\n");

        println!("{message}");

        Ok(())
    }
}
