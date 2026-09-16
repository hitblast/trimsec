use crate::{
    commands::Runnable,
    core::{
        style::Style,
        time::{TDuration, ToStringTime},
    },
};
use anyhow::{Result, anyhow};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct TrimCmd {
    /// Duration of the content (e.g. 1h2m1s, 1h1s, 2d49s).
    pub duration: String,

    /// The speed multiplier (e.g. 1.25x, 1.25).
    #[arg(short, long)]
    pub multiplier: String,
}

impl Runnable for TrimCmd {
    fn run(self, style: &Style) -> Result<()> {
        let mut duration: TDuration = TDuration::parse_str(&self.duration)
            .map_err(|e| anyhow::anyhow!("parse error: {e}"))?;

        duration
            .trim(&self.multiplier)
            .map_err(|e| anyhow!("trim error: {e}"))?;

        if duration.saved_time() <= 0.0 {
            println!("No time saved. Would finish in linear time.");
            return Ok(());
        }

        let remaining = crate::core::time::time_in_day_after(duration.seconds());
        let saved = duration.saved_time().to_string_duration();

        let message = [
            format!(
                "\nFinishes in: {} ",
                if duration.splits() > 1 {
                    format!("{duration} (all {} durations)", duration.splits())
                } else {
                    duration.to_string()
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
        Ok(())
    }
}
