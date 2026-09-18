use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    args::TCmd,
    commands::{key_set::KeySetCmd, key_show::KeyShowCmd, list::ListCmd, path::PathCmd},
};

#[derive(Parser)]
#[command(override_usage = "ts [ARGS] | [COMMAND]")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Shows the path of the configuration file.
    Path(PathCmd),
    /// Lists all entries in a YouTube playlist.
    #[command(visible_alias = "ls")]
    List(ListCmd),
    /// Command group for managing the Google Cloud Console API key.
    Key {
        #[command(subcommand)]
        command: KeySubcmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum KeySubcmd {
    /// Shows the API key that is in use, if any.
    Show(KeyShowCmd),
    /// Sets the current API key.
    Set(KeySetCmd),
}

pub fn parse_with_clap(args: &[String]) -> Result<TCmd> {
    let cli = Args::try_parse_from(std::iter::once("ts").chain(args.iter().map(String::as_str)))?;

    match cli.command {
        Command::Key { command: subcmd } => Ok(TCmd::Key { subcmd }),
        Command::List(cmd) => Ok(TCmd::List { cmd }),
        Command::Path(cmd) => Ok(TCmd::Path { cmd }),
    }
}
