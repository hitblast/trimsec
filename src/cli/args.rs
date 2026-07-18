use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{fitcheck::FitcheckCmd, list::ListCmd, trim::TrimCmd, yt::YtCmd};

#[derive(Parser)]
#[command(name = "trimsec", version, about)]
pub struct Args {
    /// Use clipboard if applicable (e.g. for link).
    #[arg(short, long, global = true)]
    pub clip: bool,

    // Selects the color mode.
    #[arg(long, value_enum, default_value_t = ColorMode::Auto, global = true)]
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
    #[command(visible_alias = "y")]
    Yt(YtCmd),
    /// Check whether content(s) fit in the day or a given budget of time.
    #[command(visible_alias = "fc")]
    Fitcheck(FitcheckCmd),
    /// Lists all entries in a YouTube playlist.
    #[command(visible_alias = "ls")]
    List(ListCmd),
}
