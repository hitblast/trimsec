use clap::{Parser, Subcommand};

use crate::commands::{fit::FitCmd, trim::TrimCmd, yt::YtCmd};

#[derive(Parser)]
#[command(name = "trimsec", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Calculate basic duration with multipliers.
    #[command(visible_alias = "t")]
    Trim(TrimCmd),
    /// Calculate for YouTube videos.
    #[command(
        visible_alias = "vid",
        override_usage = "ts yt [-l URL] -m <MULTIPLIER>\n       ts yt [--link URL] --multiplier <MULTIPLIER>            // Longer flags\n       ts yt -m <MULTIPLIER>                                   // For clipboard fallback"
    )]
    Yt(YtCmd),
    /// Check whether content(s) fit in the day or a given budget of time.
    #[command(visible_alias = "budget")]
    Fit(FitCmd),
}
