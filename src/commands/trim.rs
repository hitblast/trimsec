use crate::{
    commands::Runnable,
    core::{
        api::ApiClientManager,
        style::Style,
        time::{TDuration, ToStringTime, parse_multiplier},
        youtils::{get_youtube_api_key, get_youtube_id},
    },
};
use anyhow::{Result, anyhow, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct TrimCmd {
    /// Content to trim (a duration (e.g. 1h2m1s, 2d1h31m) or a YouTube URL).
    content: String,

    /// The speed multiplier (e.g. 1.25x, 1.25).
    multiplier: String,

    /// Max items to traverse in the given YouTube playlist (if content is a YouTube playlist URL).
    #[arg(short, long, visible_alias = "max", default_value = "0")]
    max_items: usize,
}

impl TrimCmd {
    fn yt_fallback(mut self, style: &Style) -> Result<()> {
        let key = get_youtube_api_key()?;

        let manager = ApiClientManager::new(&key);
        let id = get_youtube_id(&self.content);

        if let Some(id) = id {
            match manager.fetch_duration_from_id(&id, self.max_items) {
                Ok(dur) => {
                    self.content = dur.to_string();
                    self.max_items = 0;
                    self.run(style)?;

                    if id.is_playlist() {
                        println!("Trimmed for {} item(s).", dur.splits())
                    }
                }
                Err(e) => bail!("Failed to fetch details from URL: {e}"),
            }
        } else {
            bail!(
                "Invalid content passed! Valid contents are: any YouTube URL, any duration (e.g. 1h25m, 2d1m)"
            )
        }

        Ok(())
    }
}

impl Runnable for TrimCmd {
    fn run(self, style: &Style) -> Result<()> {
        let multiplier = parse_multiplier(&self.multiplier)
            .map_err(|e| anyhow!("multiplier parse failed: {e}"))?;

        if multiplier == 1.0 {
            println!("Would finish in linear time as used a multiplier of 1x.");
            return Ok(());
        }

        let parse_attempt = TDuration::parse_str(&self.content);

        if let Ok(mut dur) = parse_attempt {
            if self.max_items != 0 {
                bail!("--max-items can only be passed with a YouTube playlist URL.")
            }

            dur.trim(multiplier);

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
                    format!(
                        "Time in day left: {} ",
                        if remaining == 0.0 {
                            "0s".to_string()
                        } else {
                            remaining.to_string_duration()
                        }
                    )
                } else {
                    "Cannot finish today.".to_string()
                },
                format!("{}Saved {saved}!{}\n", style.boldgreen(), style.reset()),
            ]
            .join("\n");

            println!("{message}");
        } else {
            return self.yt_fallback(style);
        }

        Ok(())
    }
}
