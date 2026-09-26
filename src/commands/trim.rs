use crate::{
    args::Token,
    core::{
        context::Ctx,
        time::{TDuration, ToStringTime},
    },
    draw::point_at_arg,
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
            const TIP_EXPLICIT_PLACEMENT: &str =
                "TIP: Make the placement explicit, e.g. `1h 1.25x 3h 1.3x`.";

            match tok {
                Token::Duration(dur) => match &mut cursor_duration {
                    Some((cursor_dur, cursor_dur_idx)) => {
                        *cursor_dur += dur;
                        *cursor_dur_idx = idx;
                    }
                    None => cursor_duration = Some((dur.clone(), idx)),
                },
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
                                         {TIP_EXPLICIT_PLACEMENT}"
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
                Token::YouTube((id, max_items)) => {
                    let dur: Option<TDuration> = ctx
                        .client()
                        .ok()
                        .and_then(|f| f.fetch_duration_from_id(id, *max_items).ok());

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
                            point_at_arg(cursor_mul_idx, &ctx.style, None);
                            bail!("Unused multiplier found!\n\n {TIP_EXPLICIT_PLACEMENT}")
                        }
                    } else if let Some((_, unused_idx)) = cursor_duration {
                        point_at_arg(unused_idx, &ctx.style, None);
                        bail!("Unused duration found!\n\n {TIP_EXPLICIT_PLACEMENT}")
                    }
                }
                _ => {}
            }
        }

        Ok(runnables)
    }

    pub fn run(&mut self, ctx: &mut Ctx, remaining: &mut TDuration) -> Result<()> {
        if self.multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
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

        *remaining -= &dur;

        let message = [
            format!(
                "\n{}Finishes in: {dur}{} {}({} items){}",
                ctx.style.bold(),
                ctx.style.reset(),
                ctx.style.grey(),
                dur.splits(),
                ctx.style.reset(),
            ),
            if !remaining.is_zero() {
                format!("Time in day left: {} ", remaining)
            } else {
                format!(
                    "{}No time in day after this.{}",
                    ctx.style.boldred(),
                    ctx.style.reset()
                )
            },
            format!(
                "{}Saved {}!{}\n",
                ctx.style.boldgreen(),
                dur.saved_time().to_string_duration(),
                ctx.style.reset(),
            ),
        ]
        .join("\n");

        println!("{message}");

        Ok(())
    }
}
