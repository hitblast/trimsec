use crate::{
    args::CmdContentType,
    commands::Ctx,
    core::{
        api::ApiClient,
        time::ToStringTime,
        youtils::{YoutubeId, decide_youtube_key},
    },
};
use anyhow::{Result, bail};

pub struct TrimCmd {
    content: CmdContentType,
    multiplier: f64,
    max_items: usize,
}

impl TrimCmd {
    pub fn new(content: CmdContentType, multiplier: f64, max_items: usize) -> Self {
        Self {
            content,
            multiplier,
            max_items,
        }
    }

    fn yt_fallback(&mut self, id: &YoutubeId, ctx: &mut Ctx) -> Result<()> {
        let key = decide_youtube_key(ctx.config()?)?;
        let manager = ApiClient::new(&key);

        match manager.fetch_duration_from_id(&id, self.max_items) {
            Ok(dur) => {
                let splits = dur.splits();
                self.content = CmdContentType::Raw(dur);
                self.max_items = 0;
                self.run(ctx)?;

                if id.is_playlist() {
                    println!("Trimmed for {} item(s).", splits)
                }
            }
            Err(e) => bail!("Failed to fetch details from URL: {e}"),
        }

        Ok(())
    }

    pub fn run(&mut self, ctx: &mut Ctx) -> Result<()> {
        if self.multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
        }

        match &mut self.content {
            CmdContentType::Raw(dur) => {
                if self.max_items != 0 {
                    bail!("--max-items can only be passed with a YouTube playlist URL.")
                }

                dur.trim(self.multiplier);

                let remaining = crate::core::time::time_in_day_after(dur.seconds());
                let saved = dur.saved_time().to_string_duration();

                let message = [
                    format!(
                        "\nFinishes in: {} ",
                        if dur.splits() > 1 {
                            format!("{dur} (all {} durations)", dur.splits())
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
                        "{}Saved {saved}!{}\n",
                        ctx.style.boldgreen(),
                        ctx.style.reset()
                    ),
                ]
                .join("\n");

                println!("{message}");
            }

            CmdContentType::YouTube(e) => {
                let id = e.clone();
                return self.yt_fallback(&id, ctx);
            }
        }

        Ok(())
    }
}
