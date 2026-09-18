use std::{
    env,
    fmt::{self},
    str::FromStr,
};

use anyhow::{Result, anyhow, bail};

use crate::{
    commands::{Ctx, trim::TrimCmd},
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
    Trim { content: CmdContentType, mul: f64 },
    Fit { dur: TDuration, budget: TDuration },
    Help,
    None,
}

impl TCmd {
    pub fn run(self) -> Result<()> {
        let color_mode = should_color()?;
        let mut ctx = Ctx::new(color_mode);

        match self {
            TCmd::Trim {
                content,
                mul: multiplier,
            } => {
                let mut x = TrimCmd::new(content, multiplier, 0);
                return x.run(&mut ctx);
            }
            TCmd::Fit { dur, budget } => todo!(),
            TCmd::Help => todo!(),
            TCmd::None => todo!(),
        }
    }
}

const NONE: TCmd = TCmd::None;

pub enum ColorMode {
    Always,
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
            _ => return Err(anyhow!("color mode must be: always, auto, never")),
        };

        Ok(x)
    }
}

pub fn should_color() -> Result<ColorMode> {
    let mut args = env::args().skip(1);
    let x = if let Some(s) = args.find_map(|f| f.strip_prefix("--color=").map(str::to_owned)) {
        ColorMode::from_str(&s)?
    } else {
        ColorMode::Auto
    };
    Ok(x)
}

pub fn max_items() -> Result<usize> {
    let mut args = env::args().skip(1);
    let x = if let Some(s) = args.find_map(|f| f.strip_prefix("--max-items=").map(str::to_owned)) {
        if let Ok(y) = s.parse::<usize>() {
            y
        } else {
            bail!("invalid max-items passed: must be a positive integer")
        }
    } else {
        0
    };

    Ok(x)
}

pub fn get_cur_cmd() -> Result<TCmd> {
    let vector: Vec<String> = env::args()
        .skip(1)
        .filter(|f| !f.starts_with("--"))
        .collect();

    let mut args = vector.iter();

    if args.len() >= 3 {
        bail!("Only two positional arguments are allowed.")
    }

    if let Some(arg1) = args.next().as_deref() {
        let cmd = if let Ok(x) = TDuration::parse_str(arg1) {
            if let Some(arg2) = args.next().as_deref() {
                if let Ok(y) = TDuration::parse_str(arg2) {
                    TCmd::Fit { dur: x, budget: y }
                } else if let Ok(y) = parse_multiplier(arg2) {
                    TCmd::Trim {
                        content: CmdContentType::Raw(x),
                        mul: y,
                    }
                } else {
                    bail!("Second duration must be either a duration or a multiplier.")
                }
            } else {
                bail!(
                    "You passed in {x}, you need to pass another duration or multiplier to continue."
                )
            }
        } else if let Some(s) = get_youtube_id(arg1) {
            if let Some(arg2) = args.next().as_deref() {
                if let Ok(y) = parse_multiplier(arg2) {
                    TCmd::Trim {
                        content: CmdContentType::YouTube(s),
                        mul: y,
                    }
                } else {
                    bail!("Second argument must be either a duration or a multiplier.")
                }
            } else {
                bail!("YouTube URL verified, must also pass either a multiplier or a duration.")
            }
        } else {
            bail!("First argument must always either be a duration or a YouTube URL.")
        };

        return Ok(cmd);
    }

    Ok(NONE)
}
