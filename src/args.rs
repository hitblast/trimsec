use std::{env, str::FromStr};

use anyhow::{Result, anyhow, bail};
use clap::CommandFactory;

use crate::{
    clap::{Args, KeySubcmd, parse_with_clap},
    commands::{Ctx, fit::FitCmd, list::ListCmd, path::PathCmd, trim::TrimCmd},
    core::{
        time::{TDuration, parse_multiplier},
        youtils::{YoutubeId, get_youtube_id},
    },
};

pub enum CmdContentType {
    Raw(TDuration),
    YouTube(YoutubeId),
}

pub enum TCmd {
    Trim {
        content: CmdContentType,
        mul: f64,
    },
    Fit {
        content: CmdContentType,
        budget: Option<TDuration>,
    },
    Key {
        subcmd: KeySubcmd,
    },
    List {
        cmd: ListCmd,
    },
    Path {
        cmd: PathCmd,
    },
}

impl TCmd {
    pub fn run(self, args: &TKeywordArgs) -> Result<()> {
        let mut ctx = Ctx::new(args.color(), args);

        match self {
            TCmd::Trim {
                content,
                mul: multiplier,
            } => {
                let mut x = TrimCmd::new(content, multiplier);

                x.run(&mut ctx)
            }
            TCmd::Fit { content, budget } => {
                let x = FitCmd::new(content, budget);

                x.run(&mut ctx)
            }
            TCmd::Key { subcmd: command } => match command {
                KeySubcmd::Show(key_show_cmd) => key_show_cmd.run(&mut ctx),
                KeySubcmd::Set(key_set_cmd) => key_set_cmd.run(&mut ctx),
            },
            TCmd::List { cmd } => cmd.run(&mut ctx),
            TCmd::Path { cmd } => cmd.run(&mut ctx),
        }
    }
}

#[derive(Default)]
pub enum ColorMode {
    Always,
    #[default]
    Auto,
    Never,
}

impl FromStr for ColorMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = match s {
            "auto" => Self::Auto,
            "always" => Self::Always,
            "never" => Self::Never,
            _ => return Err(anyhow!("color mode must be one of: always, auto, never")),
        };

        Ok(x)
    }
}

#[derive(Default)]
pub struct TKeywordArgs {
    color: ColorMode,
    max_items: usize,
}

impl TKeywordArgs {
    #[must_use]
    pub fn color(&self) -> &ColorMode {
        &self.color
    }
    #[must_use]
    pub fn max_items(&self) -> usize {
        self.max_items
    }

    fn parse(args: &[String]) -> Result<(Self, Vec<String>)> {
        let mut color = None;
        let mut max_items = None;

        let mut remaining = Vec::new();

        for arg in args {
            if let Some(x) = arg.strip_prefix("--color=") {
                let None = color else {
                    bail!("Multiple --color arguments provided.")
                };
                color = Some(ColorMode::from_str(&x.to_lowercase())?);
            } else if let Some(x) = arg.strip_prefix("--max-items=") {
                let None = max_items else {
                    bail!("Multiple --max-items arguments provided.")
                };
                let Ok(y) = x.parse::<usize>() else {
                    bail!("Invalid --max-items value provided.")
                };
                max_items = Some(y);
            } else {
                remaining.push(arg.clone());
            }
        }

        Ok((
            Self {
                color: color.unwrap_or_default(),
                max_items: max_items.unwrap_or_default(),
            },
            remaining,
        ))
    }
}

pub fn get_cur_cmd() -> Result<(TKeywordArgs, TCmd)> {
    let argv: Vec<String> = env::args().skip(1).collect();

    let (kwargs, remaining) = TKeywordArgs::parse(&argv)?;

    let cmd = match remaining.first().map(String::as_str) {
        Some("key" | "help" | "list" | "ls" | "path") | None => parse_with_clap(&remaining)?,
        Some(other) => parse_deterministic(other, &remaining)?,
    };

    Ok((kwargs, cmd))
}

fn parse_deterministic(arg1: &str, args: &[String]) -> Result<TCmd> {
    if args.len() > 2 {
        Args::command().print_help()?;
        bail!("\nToo many positional arguments.")
    }

    static SARGERROR: &str = "Second argument must be either a duration or a multiplier.";

    if let Ok(x) = TDuration::parse_str(arg1) {
        match args.get(1) {
            Some(arg2) if let Ok(y) = TDuration::parse_str(arg2) => Ok(TCmd::Fit {
                content: CmdContentType::Raw(x),
                budget: Some(y),
            }),

            Some(arg2) if let Ok(y) = parse_multiplier(arg2) => Ok(TCmd::Trim {
                content: CmdContentType::Raw(x),
                mul: y,
            }),

            Some(_) => bail!(SARGERROR),

            None => Ok(TCmd::Fit {
                content: CmdContentType::Raw(x),
                budget: None,
            }),
        }
    } else if let Some(id) = get_youtube_id(arg1) {
        match args.get(1) {
            Some(arg2) if let Ok(y) = parse_multiplier(arg2) => Ok(TCmd::Trim {
                content: CmdContentType::YouTube(id),
                mul: y,
            }),

            Some(arg2) if let Ok(budget) = TDuration::parse_str(arg2) => Ok(TCmd::Fit {
                content: CmdContentType::YouTube(id),
                budget: Some(budget),
            }),

            Some(_) => bail!(SARGERROR),

            None => Ok(TCmd::Fit {
                content: CmdContentType::YouTube(id),
                budget: None,
            }),
        }
    } else {
        bail!("First argument must be a subcommand, duration, or a YouTube URL.")
    }
}
