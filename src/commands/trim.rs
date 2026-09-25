use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime},
        timeutils::time_in_day_left,
    },
};
use anyhow::{Result, bail};

pub struct TrimCmd {
    duration: TDuration,
    multiplier: f64,
}

impl TrimCmd {
    pub fn delegate(ctx: &mut Ctx, tokens: Vec<Token>) -> Result<Vec<Self>> {
        let mut cursor_duration: Option<(TDuration, usize)> = None;
        let mut cursor_multiplier: Option<(f64, usize)> = None;
        let mut runnables: Vec<Self> = Vec::new();

        let mut iterable = tokens.iter().enumerate().peekable();

        while let Some((idx, tok)) = iterable.next() {
            const LEARN_PLACEMENT_MSG: &str = "If you meant to apply different multipliers to separate durations, \
                                                make the placement explicit, e.g. `1h 1.25x 3h 1.3x`.";

            match tok {
                Token::Duration(dur) => match &mut cursor_duration {
                    Some((cursor_dur, cursor_dur_idx)) => {
                        *cursor_dur += dur;
                        *cursor_dur_idx = idx;
                    }
                    None => cursor_duration = Some((dur.clone(), idx)),
                },
                Token::BudgetDuration(_) => {
                    // This one is here for redundancy.
                    // Token::BudgetDuration(_) variants are already filtered out during the first stage.
                    bail!(
                        "Budget duration cannot be present inside an expression which prioritizes multipliers."
                    )
                }
                Token::Multiplier(new) => match &mut cursor_multiplier {
                    Some((cursor_mul, cursor_mul_idx)) => {
                        if let Some((cursor_dur, _)) = cursor_duration.take() {
                            runnables.push(Self {
                                duration: cursor_dur,
                                multiplier: *cursor_mul,
                            });
                            cursor_multiplier = Some((*new, idx));
                        } else {
                            match iterable.peek() {
                                Some((_, Token::Duration(_))) | Some((_, Token::YouTube(_))) => {
                                    println!(
                                        "Omitting unused multiplier: {cursor_mul}x from index: {cursor_mul_idx}"
                                    );
                                    cursor_multiplier = Some((*new, idx))
                                }
                                _ => {
                                    bail!(
                                        "Multiplier {cursor_mul}x (at index {cursor_mul_idx}), \
                                         {new}x (at index {idx}) given but duration does not exist.\n\n \
                                         {LEARN_PLACEMENT_MSG}"
                                    )
                                }
                            }
                        }
                    }
                    None => match cursor_duration.take() {
                        Some((duration, _)) => {
                            runnables.push(Self {
                                duration,
                                multiplier: *new,
                            });
                        }
                        None => cursor_multiplier = Some((*new, idx)),
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
                            Some((total, cursor_idx)) => {
                                *total += &dur;
                                *cursor_idx = idx;
                            }
                            None => cursor_duration = Some((dur.clone(), idx)),
                        }
                    }
                }
                Token::EOL => {
                    if let Some((cursor_mul, cursor_mul_idx)) = cursor_multiplier {
                        if let Some((duration, _)) = cursor_duration.take() {
                            runnables.push(Self {
                                duration,
                                multiplier: cursor_mul,
                            });
                        } else {
                            bail!(
                                "Unused multiplier: {cursor_mul}x at index: {cursor_mul_idx}.\n\n \
                                 {LEARN_PLACEMENT_MSG}"
                            )
                        }
                    } else if let Some((unused_dur, unused_idx)) = cursor_duration {
                        bail!("Unused duration: {unused_dur} at index: {unused_idx}")
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

        let remaining = time_in_day_left() - dur.clone();
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
            if remaining.seconds() != 0.0 {
                format!("Time in day left: {} ", remaining)
            } else {
                "No time in day after this.".to_string()
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
