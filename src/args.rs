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
    Complex((Vec<TDuration>, Vec<TYoutubeId>)),
}

pub enum TCmd {
    Trim {
        content: TCmdContent,
        mul: f64,
    },
    Fit {
        tokens: Vec<Token>,
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
    #[allow(unused)]
    Unreachable,
}

impl TCmd {
    pub fn run(self, args: TKeywordArgs) -> Result<()> {
        let mut ctx = Ctx::new(args);

        match self {
            TCmd::Trim {
                content,
                mul: multiplier,
            } => {
                let mut x = TrimCmd::new(content, multiplier);

                x.run(&mut ctx)
            }
            TCmd::Fit { tokens } => {
                let cmd = FitCmd::delegate_tokens(tokens)?;
                cmd.run(&mut ctx)
            }
            TCmd::Key { subcmd: command } => match command {
                KeySubcmd::Show(command) => command.run(&mut ctx),
                KeySubcmd::Set(command) => command.run(&mut ctx),
            },
            TCmd::List { cmd } => cmd.run(&mut ctx),
            TCmd::Path { cmd } => cmd.run(&mut ctx),
            TCmd::Unreachable => Ok(()),
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

    pub fn unset_max_items(&mut self) {
        self.max_items = 0;
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

        x.parse::<T>()
            .map_err(|e| anyhow!("Failed to parse from string: {e}"))
    }

    fn parse(args: &[String]) -> Result<(Self, Vec<String>)> {
        let mut color: Option<ColorMode> = None;
        let mut max_items: Option<usize> = None;

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
    const SUBCMDS: [&str; 8] = [
        "key",
        "list",
        "ls",
        "path",
        "-h",
        "help",
        "--help",
        "--version",
    ];

    let cmd = match remaining.first().map(String::as_str) {
        None => parse_with_clap(&remaining)?,
        Some(x) if SUBCMDS.contains(&x) => parse_with_clap(&remaining)?,
        Some(_) => parse_deterministic(&remaining)?,
    };

    Ok((kwargs, cmd))
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Duration(TDuration),
    BudgetDuration(TDuration),
    Multiplier(f64),
    YouTube(TYoutubeId),
    EOL,
}

fn parse_tokens(args: &[String]) -> (Vec<Token>, bool) {
    let mut vec: Vec<Token> = Vec::new();
    let mut trim = false;
    let mut budget_seen = false;

    for arg in args {
        if !trim && arg.starts_with("b") {
            if let Some(inner) = arg.strip_prefix("b")
                && let Ok(x) = TDuration::parse_str(inner)
            {
                if !budget_seen {
                    budget_seen = true;
                }
                vec.push(Token::BudgetDuration(x));
            }
        } else {
            if let Ok(x) = TDuration::parse_str(arg) {
                vec.push(Token::Duration(x));
            } else if let Some(id) = get_youtube_id(arg) {
                vec.push(Token::YouTube(id));
            } else if !budget_seen && let Ok(mul) = parse_multiplier(arg) {
                if !trim {
                    trim = true;
                }
                vec.push(Token::Multiplier(mul));
            }
        }
    }
    vec.push(Token::EOL);

    return (vec, trim);
}

fn parse_deterministic(args: &[String]) -> Result<TCmd> {
    let (tokens, trim) = parse_tokens(args);

    if tokens.is_empty() {
        bail!("No meaningful arguments were passed.")
    }

    let cmd = if !trim {
        TCmd::Fit { tokens }
    } else {
        TCmd::Unreachable
    };

    Ok(cmd)
}
