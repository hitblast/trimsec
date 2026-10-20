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
    clip::TClipString,
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
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        match self {
            TCmd::Trim { tokens } => {
                let cmds = TrimCmd::delegate(ctx, tokens)?;
                let mut remaining = time_in_day_left(ctx);

                for mut x in cmds {
                    x.run(ctx, &mut remaining)?
                }

                Ok(())
            }
            TCmd::Fit { tokens } => {
                let cmd = FitCmd::delegate(ctx, tokens)?;
                cmd.run()
            }
            TCmd::Key { subcmd: command } => match command {
                KeySubcmd::Show(command) => command.run(ctx),
                KeySubcmd::Set(command) => command.run(ctx),
                KeySubcmd::Unset(command) => command.run(ctx),
            },
            TCmd::List { cmd } => cmd.run(ctx),
            TCmd::Path { cmd } => cmd.run(ctx),
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

            match arg {
                _ if arg.starts_with("--color") => {
                    if color.is_some() {
                        bail!("multiple --color arguments provided.");
                    }
                    color = Some(Self::parse_kwarg(arg, "--color", &mut skippable, idx)?);
                }
                _ => {
                    remaining.push(arg.clone());
                }
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
    let mut ctx = Ctx::new(kwargs);

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
        Some(_) => parse_deter(&mut ctx, &*SESSION_ARGS, Some(&skippable))?,
    };

    Ok((ctx, cmd))
}

#[derive(Debug, PartialEq)]
pub enum ParsedArg {
    Ready(Token),
    NeedsProcessing((TYoutubeId, usize)),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Duration((TDuration, bool)),
    BudgetDuration(TDuration),
    Multiplier(f64),
    SkipThis,
    EOL,
}

struct ArgError {
    caret: Option<usize>,
    msg: String,
    inner_msg: Option<String>,
}

impl ArgError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            caret: None,
            msg: msg.into(),
            inner_msg: None,
        }
    }
    fn at(caret: usize, msg: impl Into<String>, inner_msg: Option<impl Into<String>>) -> Self {
        Self {
            caret: Some(caret),
            msg: msg.into(),
            inner_msg: inner_msg.map(|f| f.into()),
        }
    }
}

fn parse_budget(arg: &str) -> Result<ParsedArg, ArgError> {
    arg.strip_prefix('b')
        .and_then(|inner| inner.parse::<TDuration>().ok())
        .map(|e| ParsedArg::Ready(Token::BudgetDuration(e)))
        .ok_or_else(|| ArgError::new("Did you mean to set a time budget?"))
}

fn parse_n_item_cap(inner: &str) -> Result<ParsedArg, ArgError> {
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
            Some("invalid integer here"),
        )
    })?;

    let id = get_youtube_id(target, true).ok_or_else(|| {
        ArgError::new("Could not parse a valid YouTube playlist ID from the second part.")
    })?;

    if !id.is_playlist() {
        return Err(ArgError::new(
            "Item-cap target must be a playlist, not a single video.",
        ));
    }
    Ok(ParsedArg::NeedsProcessing((id, cap)))
}

fn parse_all_item_cap(inner: &str) -> Result<ParsedArg, ArgError> {
    let id = get_youtube_id(inner, true).ok_or_else(|| {
        ArgError::new("Could not parse a valid YouTube playlist ID from the inner link.")
    })?;

    if !id.is_playlist() {
        return Err(ArgError::new(
            "Inner link must be a playlist URL as `all::` is used.",
        ));
    }

    Ok(ParsedArg::NeedsProcessing((id, 0)))
}

fn parse_general(arg: &str, allow_multiplier: bool, recurse: bool) -> Result<ParsedArg, ArgError> {
    macro_rules! ready {
        ($x:expr) => {
            return Ok(ParsedArg::Ready($x))
        };
    }

    if allow_multiplier && let Ok(mul) = parse_multiplier(arg) {
        ready!(Token::Multiplier(mul))
    }
    if let Some(inner) = arg.strip_prefix("max:") {
        return parse_n_item_cap(inner);
    }
    if let Some(inner) = arg.strip_prefix("all::") {
        return parse_all_item_cap(inner);
    }
    if let Ok(x) = arg.parse::<TDuration>() {
        ready!(Token::Duration((x, false)))
    }
    if let Some(id) = get_youtube_id(arg, false) {
        return Ok(ParsedArg::NeedsProcessing((id, 0)));
    }

    if recurse {
        match TClipString::resolve(arg) {
            Ok(Some(s)) => return parse_general(&s, allow_multiplier, false),
            Err(e) => return Err(ArgError::new(e.to_string())),
            _ => {}
        }
    }

    Err(ArgError::new(if recurse {
        "Unknown argument format."
    } else {
        "Unknown clipboard content format."
    }))
}

struct PreProcessable {
    id: TYoutubeId,
    max_items: usize,
    idx: usize,
}

fn parse_tokens(
    ctx: &mut Ctx,
    args: &Vec<String>,
    skippable: Option<&HashSet<usize>>,
) -> Result<(Vec<Token>, bool)> {
    let mut vec: Vec<Token> = Vec::new();
    let mut trim = false;
    let mut budget_seen = false;
    let mut invalid_toks: usize = 0;

    let mut preprocessable: Vec<PreProcessable> = Vec::new();

    for (idx, arg) in args.into_iter().enumerate() {
        if let Some(skippable) = skippable
            && skippable.contains(&idx)
        {
            vec.push(SkipThis);
            continue;
        }

        let parsed = if !trim && arg.starts_with('b') {
            parse_budget(arg)
        } else {
            parse_general(arg, !budget_seen, true)
        };

        match parsed {
            Ok(p) => match p {
                ParsedArg::Ready(token) => {
                    match token {
                        Token::BudgetDuration(_) => budget_seen = true,
                        Token::Multiplier(_) => trim = true,
                        _ => {}
                    }
                    vec.push(token);
                }
                ParsedArg::NeedsProcessing((id, max_items)) => {
                    let pre = PreProcessable { id, max_items, idx };
                    preprocessable.push(pre);
                }
            },
            Err(e) => {
                point_at_arg(idx, &ctx.style, e.caret, false, e.inner_msg.as_deref());
                println!("{}", e.msg);
                invalid_toks += 1;
            }
        }
    }

    if invalid_toks > 0 {
        bail!("{invalid_toks} invalid tokens found.")
    }

    if !preprocessable.is_empty() {
        let client = match ctx.client() {
            Ok(c) => c,
            Err(e) => bail!("Failed to prepare client: {e}"),
        };

        let mapped_preprocessed = preprocessable
            .iter()
            .map(|pre| {
                client
                    .fetch_duration_from_id(&pre.id, pre.max_items)
                    .map(|f| (f, pre.idx))
                    .map_err(|e| anyhow::anyhow!(e))
            })
            .collect::<Vec<Result<(TDuration, usize)>>>();

        for processed in mapped_preprocessed {
            match processed {
                Ok((dur, idx)) => vec.insert(idx, Token::Duration((dur, true))),
                Err(e) => {
                    bail!("Failed to process one or more YouTube URLs: {e}")
                }
            }
        }
    }

    vec.push(Token::EOL);

    Ok((vec, trim))
}

/// Core deterministic-parser entrypoint for trimsec.
///
/// This is used by both the non-interactive, and the interactive command-line interface parts.
pub fn parse_deter(
    ctx: &mut Ctx,
    args: &Vec<String>,
    skippable: Option<&HashSet<usize>>,
) -> Result<TCmd> {
    let (tokens, trim): (Vec<Token>, bool) = parse_tokens(ctx, args, skippable)?;

    let cmd: TCmd = if !trim {
        TCmd::Fit { tokens }
    } else {
        TCmd::Trim { tokens }
    };

    Ok(cmd)
}
