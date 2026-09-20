use crate::{
    args::TCmdContent,
    core::{context::Ctx, time::ToStringTime, youtils::TYoutubeId},
};
use anyhow::{Result, bail};

pub struct TrimCmd {
    content: TCmdContent,
    multiplier: f64,
}

impl TrimCmd {
    #[must_use]
    pub fn new(content: TCmdContent, multiplier: f64) -> Self {
        Self {
            content,
            multiplier,
        }
    }

    fn yt_fallback(&mut self, id: &TYoutubeId, ctx: &mut Ctx) -> Result<()> {
        let max = ctx.kwargs.max_items();
        match ctx.client()?.fetch_duration_from_id(id, max) {
            Ok(dur) => {
                self.content = TCmdContent::Raw(dur);
                self.run(ctx)?;
            }
            Err(e) => bail!("Fetching URL failed: {e}"),
        }

        Ok(())
    }

    pub fn run(&mut self, ctx: &mut Ctx) -> Result<()> {
        if self.multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
        }

        match &mut self.content {
            TCmdContent::Raw(dur) => {
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
                        ctx.style.boldgreen(),
                        ctx.style.reset(),
                        dur.splits()
                    ),
                ]
                .join("\n");

                println!("{message}");
            }

            TCmdContent::YouTube(e) => {
                let id = e.clone();
                return self.yt_fallback(&id, ctx);
            }
        }

        Ok(())
    }
}
