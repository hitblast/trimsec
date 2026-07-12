use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{fitcheck::FitcheckCmd, trim::TrimCmd, yt::YtCmd};

#[derive(Parser)]
#[command(name = "trimsec", version, about)]
pub struct Args {
    /// Disable grabbing links from clipboard.
    #[arg(short, long)]
    pub no_clip: bool,

    // Selects the color mode.
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ColorMode {
    Always,
    Auto,
    Never,
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
    #[command(visible_alias = "fc")]
    Fitcheck(FitcheckCmd),
}
