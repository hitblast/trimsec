use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    args::TCmd,
    commands::{key_set::KeySetCmd, key_show::KeyShowCmd, list::ListCmd, path::PathCmd},
};

#[derive(Parser)]
#[command(
    version,
    override_usage = r#"
ts [ARGS | COMMAND]

Dynamic Arguments:
  ts 1h24m 1.25x                        # trim mode
  ts https://youtube.com/... 1.25x      # trim mode (yt)

  ts 1h24m 3h                           # fit-check (fixed duration)
  ts https://youtu.be/... 3h            # fit-check (yt, fixed duration)
  ts 1h24m                              # fit-check (remaining day as duration)
  ts https://youtu.be/...               # fit-check (yt, remaining day as duration)

Keyword Arguments:
  1. Playlist traverse limit:
    --max-items=<uint>                  e.g. --max-items=7
  2. Color mode:
    --color=<ColorMode>                 e.g. --color=auto
                                             (modes: always, auto, never)
"#
)]
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
