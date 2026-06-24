use clap::Args;

use crate::formatting::*;
use crate::{commands::Runnable, core::time::TimeConfig};
use anyhow::{Result, bail};

#[derive(Debug, Default, Args)]
pub struct TrimCmd {
    /// Duration of the content (e.g. 1h2m1s, 1h1s, 2d49s).
    duration: String,
    /// The speed multiplier (e.g. 1.25x, 1.25).
    multiplier: String,
}

impl Runnable for TrimCmd {
    fn run(&self) -> Result<()> {
        let config = TimeConfig::new(&self.duration, &self.multiplier);

        match config {
            Ok(cfg) => {
                let trimmed = cfg.trim();

                match trimmed {
                    Ok((new_duration, time_saved, splits)) => {
                        if time_saved > 0.0 {
                            let parsed = crate::core::time::parse_time(new_duration);

                            let message = format!(
                                "\nFinishes in: {} ",
                                if splits > 1 {
                                    format!("{parsed} (all {splits} durations)")
                                } else {
                                    parsed
                                }
                            );
                            println!("{}", message);
                        } else {
                            println!("No time saved. Would finish in linear time.");
                            return Ok(());
                        }

                        let remaining = crate::core::time::calculate_remaining(new_duration);
                        println!(
                            "Time in day left: {} ",
                            if remaining == 0.0 {
                                "0s".to_string()
                            } else {
                                crate::core::time::parse_time(remaining)
                            }
                        );

                        if time_saved > 0.0 {
                            let parsed = crate::core::time::parse_time(time_saved);
                            println!("{GREEN}{BOLD}Saved {parsed}!{RESET}\n");
                        }
                    }
                    Err(e) => bail!("trim error: {e}"),
                }
            }
            Err(e) => bail!("time configuration error: {e}"),
        }
        Ok(())
    }
}
