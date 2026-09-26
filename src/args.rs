use std::{collections::HashSet, env, fmt::Display, str::FromStr};

use anyhow::{Result, anyhow, bail};

use crate::{
    clap::{KeySubcmd, parse_with_clap},
    commands::{fit::FitCmd, list::ListCmd, path::PathCmd, trim::TrimCmd},
    core::{
        context::Ctx,
        time::{TDuration, parse_multiplier},
        timeutils::time_in_day_left,
        youtils::{TYoutubeId, get_youtube_id},
    },
    draw::point_at_arg,
};

pub enum TCmdContent {
    Raw(TDuration),
    YouTube(TYoutubeId),
    Complex((Vec<TDuration>, Vec<TYoutubeId>)),
}

pub enum TCmd {
    Trim { tokens: Vec<Token> },
    Fit { tokens: Vec<Token> },
    Key { subcmd: KeySubcmd },
    List { cmd: ListCmd },
    Path { cmd: PathCmd },
}

impl TCmd {
    pub fn run(self, args: TKeywordArgs) -> Result<()> {
        let mut ctx: Ctx = Ctx::new(args);

        let token_check = |tokens: &Vec<Token>| -> Result<()> {
            let mut should_bail = false;

            for tok in tokens {
                if let Token::Invalid((idx, reason, specific_idx)) = tok {
                    point_at_arg(*idx, &ctx.style, *specific_idx);
                    println!("{reason}");
                    should_bail = true;
                }
            }

            if should_bail {
                bail!("invalid tokens found.")
            }

            Ok(())
        };

        match self {
            TCmd::Trim { tokens } => {
                token_check(&tokens)?;

                let cmds = TrimCmd::delegate(&mut ctx, tokens)?;
                let mut remaining = time_in_day_left();

                for mut x in cmds {
                    x.run(&mut ctx, &mut remaining)?
                }

                Ok(())
            }
            TCmd::Fit { tokens } => {
                token_check(&tokens)?;

                let cmd = FitCmd::delegate(&mut ctx, tokens)?;
                cmd.run()
            }
            TCmd::Key { subcmd: command } => match command {
                KeySubcmd::Show(command) => command.run(&mut ctx),
                KeySubcmd::Set(command) => command.run(&mut ctx),
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
}

impl TKeywordArgs {
    #[must_use]
    pub fn color(&self) -> &ColorMode {
        &self.color
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
            bail!("missing value for keyword argument: {keyword}")
        };

        skippable.insert(idx + 1);

        x.parse::<T>()
            .map_err(|e| anyhow!("Failed to parse from string: {e}"))
    }

    fn parse(args: &[String]) -> Result<(Self, Vec<String>)> {
        let mut color: Option<ColorMode> = None;

        let mut remaining = Vec::new();
        let mut skippable: HashSet<usize> = HashSet::new();

        for (idx, arg) in args.iter().enumerate() {
            if skippable.contains(&idx) {
                continue;
            }

            if arg.starts_with("--color") {
                let None = color else {
                    bail!("multiple --color arguments provided.")
                };

                let x: ColorMode = Self::parse_kwarg(arg, "--color", args, &mut skippable, idx)?;
                color = Some(x);
            } else {
                remaining.push(arg.clone());
            }
        }

        Ok((
            Self {
                color: color.unwrap_or_default(),
            },
            remaining,
        ))
    }
}

pub fn get_cur_cmd() -> Result<(TKeywordArgs, TCmd)> {
    let argv: Vec<String> = env::args().skip(1).collect();

    let (kwargs, remaining): (TKeywordArgs, Vec<String>) = TKeywordArgs::parse(&argv)?;
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

    let cmd: TCmd = match remaining.first().map(String::as_str) {
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
    YouTube((TYoutubeId, usize)),
    Invalid((usize, String, Option<usize>)),
    EOL,
}

macro_rules! invalid {
    ($x:expr, $($arg:tt)*) => {
        Token::Invalid(($x, format!($($arg)*), None))
    };
}

fn parse_tokens(args: &[String]) -> (Vec<Token>, bool) {
    let mut vec: Vec<Token> = Vec::new();
    let mut trim = false;
    let mut budget_seen = false;

    for (idx, arg) in args.iter().enumerate() {
        if !trim && arg.starts_with("b") {
            let tok = if let Some(inner) = arg.strip_prefix("b")
                && let Ok(x) = TDuration::parse_str(inner)
            {
                if !budget_seen {
                    budget_seen = true;
                }
                Token::BudgetDuration(x)
            } else {
                invalid!(idx, "Did you mean to set a time-budget?")
            };

            vec.push(tok);
        } else {
            let tok = if let Ok(x) = TDuration::parse_str(arg) {
                Token::Duration(x)
            } else if let Some(id) = get_youtube_id(arg) {
                Token::YouTube((id, 0))
            } else if let Some(inner) = arg.strip_prefix("max:") {
                let split: Vec<&str> = inner.split("::").collect();

                let tok = if split.len() == 2 {
                    if let Some(first) = split.get(0).and_then(|f| f.parse::<usize>().ok()) {
                        if let Some(second) = split.get(1).and_then(|f| get_youtube_id(f)) {
                            if second.is_playlist() {
                                Token::YouTube((second.clone(), first))
                            } else {
                                invalid!(
                                    idx,
                                    "Item-cap target must be a playlist, not a single video."
                                )
                            }
                        } else {
                            invalid!(
                                idx,
                                "Could not parse a valid YouTube id/playlist from the second part."
                            )
                        }
                    } else {
                        Token::Invalid((
                            idx,
                            "Item-cap must be a positive number, e.g. max:10::<playlist>."
                                .to_string(),
                            Some(4),
                        ))
                    }
                } else {
                    invalid!(
                        idx,
                        "Expected exactly 2 parts separated by '::' for an item-cap (got {}).",
                        split.len()
                    )
                };

                tok
            } else if !budget_seen && let Ok(mul) = parse_multiplier(arg) {
                if !trim {
                    trim = true;
                }
                Token::Multiplier(mul)
            } else {
                invalid!(idx, "Unknown argument format.")
            };

            vec.push(tok);
        }
    }
    vec.push(Token::EOL);

    (vec, trim)
}

fn parse_deterministic(args: &[String]) -> Result<TCmd> {
    let (tokens, trim): (Vec<Token>, bool) = parse_tokens(args);

    if let Some(Token::EOL) = tokens.get(0) {
        bail!("no meaningful arguments were passed.")
    }

    let cmd: TCmd = if !trim {
        TCmd::Fit { tokens }
    } else {
        TCmd::Trim { tokens }
    };

    Ok(cmd)
}
