use std::{env, str::FromStr};

use anyhow::{Result, anyhow, bail};

use crate::{
    commands::{Ctx, fit::FitCmd, trim::TrimCmd},
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
    Help,
    None,
}

impl TCmd {
    pub fn run(self) -> Result<()> {
        let color_mode = should_color()?;
        let max_items = max_items()?;
        let mut ctx = Ctx::new(color_mode);

        match self {
            TCmd::Trim {
                content,
                mul: multiplier,
            } => {
                let mut x = TrimCmd::new(content, multiplier, max_items);
                return x.run(&mut ctx);
            }
            TCmd::Fit { content, budget } => {
                let x = FitCmd::new(content, budget, max_items);
                return x.run(&mut ctx);
            }
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
        static SARGERROR: &str = "Second argument must be either a duration or a multiplier.";

        let cmd = if let Ok(x) = TDuration::parse_str(arg1) {
            if let Some(arg2) = args.next().as_deref() {
                if let Ok(y) = TDuration::parse_str(arg2) {
                    TCmd::Fit {
                        content: CmdContentType::Raw(x),
                        budget: Some(y),
                    }
                } else if let Ok(y) = parse_multiplier(arg2) {
                    TCmd::Trim {
                        content: CmdContentType::Raw(x),
                        mul: y,
                    }
                } else {
                    bail!(SARGERROR)
                }
            } else {
                TCmd::Fit {
                    content: CmdContentType::Raw(x),
                    budget: None,
                }
            }
        } else if let Some(id) = get_youtube_id(arg1) {
            if let Some(arg2) = args.next().as_deref() {
                if let Ok(y) = parse_multiplier(arg2) {
                    TCmd::Trim {
                        content: CmdContentType::YouTube(id),
                        mul: y,
                    }
                } else if let Ok(budget) = TDuration::parse_str(arg2) {
                    TCmd::Fit {
                        content: CmdContentType::YouTube(id),
                        budget: Some(budget),
                    }
                } else {
                    bail!(SARGERROR)
                }
            } else {
                TCmd::Fit {
                    content: CmdContentType::YouTube(id),
                    budget: None,
                }
            }
        } else {
            bail!("First argument must always either be a duration or a YouTube URL.")
        };

        return Ok(cmd);
    }

    Ok(NONE)
}
