use std::{
    collections::HashSet,
    env,
    fmt::Display,
    io::{self, IsTerminal, Read},
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{Result, anyhow, bail};

use crate::{
    args::Token::SkipThis,
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
    pub fn run(self, mut ctx: Ctx) -> Result<()> {
        match self {
            TCmd::Trim { tokens } => {
                let cmds = TrimCmd::delegate(&mut ctx, tokens)?;
                let mut remaining = time_in_day_left();

                for mut x in cmds {
                    x.run(&mut ctx, &mut remaining)?
                }

                Ok(())
            }
            TCmd::Fit { tokens } => {
                let cmd = FitCmd::delegate(&mut ctx, tokens)?;
                cmd.run()
            }
            TCmd::Key { subcmd: command } => match command {
                KeySubcmd::Show(command) => command.run(&mut ctx),
                KeySubcmd::Set(command) => command.run(&mut ctx),
                KeySubcmd::Unset(command) => command.run(&mut ctx),
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
            s => {
                return Err(anyhow!(
                    "color mode must be one of: always, auto, never (got: {s})"
                ));
            }
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
        skippable: &mut HashSet<usize>,
        idx: usize,
    ) -> Result<T>
    where
        T: FromStr,
        T::Err: Display,
    {
        let x = if let Some(x) = arg.strip_prefix(&format!("{keyword}=")) {
            skippable.insert(idx);
            x
        } else if arg == keyword
            && let Some(argval) = SESSION_ARGS.get(idx + 1)
        {
            skippable.extend([idx, idx + 1]);
            argval.as_str()
        } else {
            bail!("missing value for keyword argument: {keyword}")
        };

        x.parse::<T>()
            .map_err(|e| anyhow!("Failed to parse from string: {e}"))
    }

    fn parse() -> Result<(Self, Vec<String>, HashSet<usize>)> {
        let mut color: Option<ColorMode> = None;

        let mut remaining = Vec::new();
        let mut skippable: HashSet<usize> = HashSet::new();

        for (idx, arg) in SESSION_ARGS.iter().enumerate() {
            if skippable.contains(&idx) {
                continue;
            }

            if arg.starts_with("--color") {
                let None = color else {
                    bail!("multiple --color arguments provided.")
                };

                let x: ColorMode = Self::parse_kwarg(arg, "--color", &mut skippable, idx)?;
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
            skippable,
        ))
    }
}

pub static SESSION_ARGS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut argv: Vec<String> = env::args().skip(1).collect();

    if !io::stdin().is_terminal() {
        let mut stdin = String::new();

        #[allow(clippy::expect_used)]
        io::stdin()
            .read_to_string(&mut stdin)
            .expect("failed to read stdin inside lazylock");

        let mut stdin_args: Vec<String> = stdin.split_whitespace().map(|f| f.to_string()).collect();
        argv.append(&mut stdin_args);
    }

    argv
});

pub fn fetch_ctx_and_runnable() -> Result<(Ctx, TCmd)> {
    let (kwargs, remaining, skippable) = TKeywordArgs::parse()?;
    let ctx = Ctx::new(kwargs);

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
        Some(_) => parse_deterministic(&ctx, &skippable)?,
    };

    Ok((ctx, cmd))
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Duration(TDuration),
    BudgetDuration(TDuration),
    Multiplier(f64),
    YouTube((TYoutubeId, usize)),
    SkipThis,
    EOL,
}

struct ArgError {
    caret: Option<usize>,
    msg: String,
}

impl ArgError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            caret: None,
            msg: msg.into(),
        }
    }
    fn at(caret: usize, msg: impl Into<String>) -> Self {
        Self {
            caret: Some(caret),
            msg: msg.into(),
        }
    }
}

fn parse_budget(arg: &str) -> Result<Token, ArgError> {
    arg.strip_prefix('b')
        .and_then(|inner| TDuration::parse_str(inner).ok())
        .map(Token::BudgetDuration)
        .ok_or_else(|| ArgError::new("Did you mean to set a time budget?"))
}

fn parse_item_cap(inner: &str) -> Result<Token, ArgError> {
    let parts: Vec<&str> = inner.split("::").collect();
    let [cap, target] = parts.as_slice() else {
        return Err(ArgError::new(format!(
            "Expected exactly 2 parts separated by '::' for an item-cap (got {}).",
            parts.len()
        )));
    };

    let cap = cap.parse::<usize>().map_err(|_| {
        ArgError::at(
            4,
            "Item-cap must be a positive number, e.g. max:10::<playlist>.",
        )
    })?;

    let id = get_youtube_id(target).ok_or_else(|| {
        ArgError::new("Could not parse a valid YouTube id/playlist from the second part.")
    })?;

    if !id.is_playlist() {
        return Err(ArgError::new(
            "Item-cap target must be a playlist, not a single video.",
        ));
    }
    Ok(Token::YouTube((id, cap)))
}

fn parse_general(arg: &str, allow_multiplier: bool) -> Result<Token, ArgError> {
    if let Ok(x) = TDuration::parse_str(arg) {
        return Ok(Token::Duration(x));
    }
    if let Some(id) = get_youtube_id(arg) {
        return Ok(Token::YouTube((id, 0)));
    }
    if let Some(inner) = arg.strip_prefix("max:") {
        return parse_item_cap(inner);
    }
    if allow_multiplier && let Ok(mul) = parse_multiplier(arg) {
        return Ok(Token::Multiplier(mul));
    }
    Err(ArgError::new("Unknown argument format."))
}

fn parse_tokens(ctx: &Ctx, skippable: &HashSet<usize>) -> Result<(Vec<Token>, bool)> {
    let mut vec: Vec<Token> = Vec::new();
    let mut trim = false;
    let mut budget_seen = false;
    let mut invalid_toks: usize = 0;

    for (idx, arg) in SESSION_ARGS.iter().enumerate() {
        if skippable.contains(&idx) {
            vec.push(SkipThis);
            continue;
        }

        let parsed = if !trim && arg.starts_with('b') {
            parse_budget(arg)
        } else {
            parse_general(arg, !budget_seen)
        };

        let tok = match parsed {
            Ok(tok) => {
                match tok {
                    Token::BudgetDuration(_) => budget_seen = true,
                    Token::Multiplier(_) => trim = true,
                    _ => {}
                }
                tok
            }
            Err(e) => {
                point_at_arg(idx, &ctx.style, e.caret, false);
                println!("{}", e.msg);
                invalid_toks += 1;
                SkipThis
            }
        };

        vec.push(tok);
    }

    if invalid_toks > 0 {
        bail!("{invalid_toks} invalid tokens found.")
    }
    vec.push(Token::EOL);

    Ok((vec, trim))
}

fn parse_deterministic(ctx: &Ctx, skippable: &HashSet<usize>) -> Result<TCmd> {
    let (tokens, trim): (Vec<Token>, bool) = parse_tokens(ctx, skippable)?;

    let cmd: TCmd = if !trim {
        TCmd::Fit { tokens }
    } else {
        TCmd::Trim { tokens }
    };

    Ok(cmd)
}
