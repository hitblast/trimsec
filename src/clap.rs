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

Usage Syntax:
  ts [ARGS]:
    ts 1h24m 2h 1.25x                        # trim mode (implicit combination)
    ts 1h24m+2h+1m 1.25x                                 (explicit comb.)
    ts 1h24m 1.25x <URL> 2h30m 1.25x ...                 (arbitrary arguments)

    ts 1h24m                                 # fit-check (remaining day-time as budget)
    ts 1h24m+20m+30m b12h                                (explicit combination and budget)
    ts 1h24m 2h                                          (second argument as budget -> 2h)
    ts <URL> 3h20m <URL> ...                             (arbitrary arguments)
    ts <URL> 1h13m1.5s <URL> ... b12h                    (explicit 12h budget via 'b' prefix)

  ts [COMMAND]: See commands below.

Substitute:
  <URL>    -> Any YouTube URL (videos/music/Shorts).

Keyword Arguments:
  1. Color mode:
    --color <ColorMode>                 e.g. --color auto
                                         or, --color=auto
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
