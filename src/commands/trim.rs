use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{style::Style, time::TimeConfig},
};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct TrimCmd {
    /// Duration of the content (e.g. 1h2m1s, 1h1s, 2d49s).
    pub duration: String,

    /// The speed multiplier (e.g. 1.25x, 1.25).
    pub multiplier: String,
}

impl Runnable for TrimCmd {
    fn run(self, _: &Flags, style: &Style) -> Result<()> {
        let cfg = TimeConfig::new(&self.duration, &self.multiplier)
            .map_err(|e| anyhow::anyhow!("Time configuration error: {e}"))?;

        let (new_duration, time_saved, splits) =
            cfg.trim().map_err(|e| anyhow::anyhow!("Trim error: {e}"))?;

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
            if remaining != 0.0 {
                format!(
                    "Time in day left: {} ",
                    if remaining == 0.0 {
                        "0s".to_string()
                    } else {
                        crate::core::time::parse_time(remaining)
                    }
                )
            } else {
                "Cannot finish today".to_string()
            },
            format!(
                "{}{}Saved {saved}!{}\n",
                style.green, style.bold, style.reset
            ),
        ]
        .join("\n");

        println!("{message}");
        Ok(())
    }
}
