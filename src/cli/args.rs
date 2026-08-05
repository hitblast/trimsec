use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{
    fits::FitsCmd, key_set::KeySetCmd, key_show::KeyShowCmd, list::ListCmd, path::PathCmd,
    trim::TrimCmd, yt::YtCmd,
};

#[derive(Parser)]
#[command(name = "trimsec", version, about)]
pub struct Args {
    /// Use clipboard if applicable (e.g. for grabbing a link).
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
    Trim(TrimCmd),
    /// Calculate for YouTube videos.
    Yt(YtCmd),
    /// Check whether content(s) fit in the day or a given budget of time.
    Fits(FitsCmd),
    /// Lists all entries in a YouTube playlist.
    #[command(visible_alias = "ls")]
    List(ListCmd),
    /// Command group for managing the Google Cloud Console API key.
    Key {
        #[command(subcommand)]
        command: KeySubcmd,
    },
    /// Shows the path of the configuration file.
    Path(PathCmd),
}

#[derive(Subcommand, Debug)]
pub enum KeySubcmd {
    /// Shows the API key that is in use, if any.
    Show(KeyShowCmd),
    /// Sets the current API key.
    Set(KeySetCmd),
}
