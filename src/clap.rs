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

Usage:
    ts [ARGS]:
        ts 1h24m 2h 1.25x
            # trim mode (implicit combination)

        ts 1h24m+2h+1m 1.25x
            # trim mode (explicit combination)

        ts 1h24m 1.25x <URL> 2h30m 1.25x ...
            # arbitrary arguments

        ts 1h24m
            # fit-check (remaining time until midnight as budget)

        ts 1h24m+20m+30m b12h
            # explicit combination and explicit budget

        ts 1h24m 2h
            # second duration argument is interpreted as the budget

        ts <URL> 3h20m <URL> ...
            # arbitrary arguments

        ts <URL> 1h13m1.5s <URL> ... b12h
            # explicit 12h budget via 'b' prefix

    ts [COMMAND]:
        See commands below.


Argument Syntax:

    <DURATION>
    e.g. 1h24m, 2h30m, 1m13.5s

    <SPEED>
    e.g. 1.25x

    <BUDGET>
    b<DURATION>
    e.g. b12h

    <COMBINATION>
    <DURATION>+<DURATION>+...
    e.g. 1h24m+20m+30m

    <URL>
    Any YouTube URL (videos/music/Shorts).

    <PLAYLIST>
    A YouTube playlist URL.


Argument Interpretation:

    - Adjacent duration arguments are implicitly combined when used in
    trim mode.

        ts 1h24m 2h 1.25x
        => (1h24m + 2h) at 1.25x

    - Durations may be explicitly combined with '+'.

        ts 1h24m+2h+1m 1.25x
        => (1h24m + 2h + 1m) at 1.25x

    - A standalone second duration argument is interpreted as the budget
    when the command is in fit-check mode.

        ts 1h24m 2h
        => check 1h24m against a 2h budget

    - A budget prefixed with 'b' explicitly specifies the budget.

        ts 1h24m+20m+30m b12h
        => check (1h24m + 20m + 30m) against a 12h budget

    - If no budget is specified, fit-check uses the remaining time until
    midnight as the budget.

    - A <SPEED> argument applies to the duration/combination immediately
    preceding it.


Specific Arg-Syntax:

    max:<n>::<PLAYLIST>
    Only traverse the first n items in a YouTube playlist.

    Example:
        max:10::https://youtube.com/playlist?list=...


Keyword Arguments:

    1. Color mode:

    --color <ColorMode>
        e.g. --color auto

    or:

    --color=<ColorMode>
        e.g. --color=auto

    Modes:
        always
        auto
        never

    Keyword arguments may appear anywhere in the argument list.


Commands:

    See commands below.
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
