use crate::core::time::TimeConfig;
use crate::formatting::*;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct TrimCmd {
    /// Duration of the content (e.g. 1h2m1s, 1h1s, 2d49s).
    pub duration: String,

    /// The speed multiplier (e.g. 1.25x, 1.25).
    pub multiplier: String,
}

impl TrimCmd {
    pub fn run(&self) -> Result<()> {
        let cfg = TimeConfig::new(&self.duration, &self.multiplier)
            .map_err(|e| anyhow::anyhow!("time configuration error: {e}"))?;

        let (new_duration, time_saved, splits) =
            cfg.trim().map_err(|e| anyhow::anyhow!("trim error: {e}"))?;

        if time_saved <= 0.0 {
            println!("No time saved. Would finish in linear time.");
            return Ok(());
        }

        let parsed = crate::core::time::parse_time(new_duration);
        let remaining = crate::core::time::time_in_day_after(new_duration);
        let saved = crate::core::time::parse_time(time_saved);

        let message = [
            format!(
                "\nFinishes in: {} ",
                if splits > 1 {
                    format!("{parsed} (all {splits} durations)")
                } else {
                    parsed
                }
            ),
            format!(
                "Time in day left: {} ",
                if remaining == 0.0 {
                    "0s".to_string()
                } else {
                    crate::core::time::parse_time(remaining)
                }
            ),
            format!("{GREEN}{BOLD}Saved {saved}!{RESET}\n"),
        ]
        .join("\n");

        println!("{message}");
        Ok(())
    }
}
