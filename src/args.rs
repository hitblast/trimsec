use std::{collections::HashSet, env, fmt::Display, str::FromStr};

use anyhow::{Result, anyhow, bail};

use crate::{
    clap::{KeySubcmd, parse_with_clap},
    commands::{fit::FitCmd, list::ListCmd, path::PathCmd, trim::TrimCmd},
    core::{
        context::Ctx,
        time::{TDuration, parse_multiplier},
        youtils::{TYoutubeId, get_youtube_id},
    },
};

pub enum TCmdContent {
    Raw(TDuration),
    YouTube(TYoutubeId),
}

pub enum TCmd {
    Trim {
        content: TCmdContent,
        mul: f64,
    },
    Fit {
        content: TCmdContent,
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

    fn parse_kwarg<T>(
        arg: &str,
        keyword: &str,
        args: &[String],
        skippable: &mut HashSet<usize>,
        idx: usize,
    ) -> Result<T>
    where
        T: FromStr,
        T::Err: Display,
    {
        let x = if let Some(x) = arg.strip_prefix(&format!("{keyword}=")) {
            x
        } else if arg == keyword
            && let Some(argval) = args.get(idx + 1)
        {
            argval.as_str()
        } else {
            bail!("Missing value for keyword argument: {keyword}")
        };

        skippable.insert(idx + 1);

        Ok(x.parse::<T>()
            .map_err(|e| anyhow!("Failed to parse from string: {e}"))?)
    }

    fn parse<'a>(args: &[String]) -> Result<(Self, Vec<String>)> {
        let mut color = None;
        let mut max_items = None;

        let mut remaining = Vec::new();
        let mut skippable: HashSet<usize> = HashSet::new();

        for (idx, arg) in args.iter().enumerate() {
            if skippable.contains(&idx) {
                continue;
            }

            if arg.starts_with("--color") {
                let None = color else {
                    bail!("Multiple --color arguments provided.")
                };

                let x: ColorMode = Self::parse_kwarg(arg, "--color", args, &mut skippable, idx)?;
                color = Some(x);
            } else if arg.starts_with("--max-items") {
                let None = max_items else {
                    bail!("Multiple --max-items arguments provided.")
                };

                let x: usize = Self::parse_kwarg(arg, "--max-items", args, &mut skippable, idx)?;
                max_items = Some(x);
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
        Some("key" | "help" | "-h" | "--help" | "list" | "ls" | "path" | "--version") | None => {
            parse_with_clap(&remaining)?
        }
        Some(first) => parse_deterministic(first, &remaining)?,
    };

    Ok((kwargs, cmd))
}

fn parse_deterministic(first: &str, args: &[String]) -> Result<TCmd> {
    static SARGERROR: &str = "Second argument must be either a duration or a multiplier.";

    if let Ok(x) = TDuration::parse_str(first) {
        match args.get(1) {
            Some(arg2) if let Ok(y) = TDuration::parse_str(arg2) => Ok(TCmd::Fit {
                content: TCmdContent::Raw(x),
                budget: Some(y),
            }),

            Some(arg2) if let Ok(y) = parse_multiplier(arg2) => Ok(TCmd::Trim {
                content: TCmdContent::Raw(x),
                mul: y,
            }),

            Some(_) => bail!(SARGERROR),

            None => Ok(TCmd::Fit {
                content: TCmdContent::Raw(x),
                budget: None,
            }),
        }
    } else if let Some(id) = get_youtube_id(first) {
        match args.get(1) {
            Some(arg2) if let Ok(y) = parse_multiplier(arg2) => Ok(TCmd::Trim {
                content: TCmdContent::YouTube(id),
                mul: y,
            }),

            Some(arg2) if let Ok(budget) = TDuration::parse_str(arg2) => Ok(TCmd::Fit {
                content: TCmdContent::YouTube(id),
                budget: Some(budget),
            }),

            Some(_) => bail!(SARGERROR),

            None => Ok(TCmd::Fit {
                content: TCmdContent::YouTube(id),
                budget: None,
            }),
        }
    } else {
        bail!("First argument must be a subcommand, duration, or a YouTube URL.")
    }
}
