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
        let args = TKeywordArgs::traverse()?;
        let mut ctx = Ctx::new(&args.color);

        match self {
            TCmd::Trim {
                content,
                mul: multiplier,
            } => {
                let mut x = TrimCmd::new(content, multiplier, args.max_items());
                x.run(&mut ctx)
            }
            TCmd::Fit { content, budget } => {
                let x = FitCmd::new(content, budget, args.max_items());
                x.run(&mut ctx)
            }
            TCmd::Help => todo!(),
            TCmd::None => todo!(),
        }
    }
}

const NONE: TCmd = TCmd::None;

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
            _ => return Err(anyhow!("color mode must be: always, auto, never")),
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

    fn traverse() -> Result<Self> {
        let mut val = Self::default();
        let mut args = env::args().skip(1);

        let mut color_mode = None;
        let mut max_items = None;

        while let Some(arg) = args.next().as_deref() {
            if let Some(x) = arg.strip_prefix("--color=") {
                let None = color_mode else {
                    bail!("Multiple --color arguments provided.")
                };
                let mode = ColorMode::from_str(x)?;
                color_mode = Some(mode);
            } else if let Some(x) = arg.strip_prefix("--max-items=") {
                if max_items.is_none() {
                    let Ok(y) = x.parse::<usize>() else {
                        bail!("Invalid --max-items value provided.")
                    };
                    max_items = Some(y);
                }
            } else {
                continue;
            }
        }

        if let Some(c) = color_mode {
            val.color = c;
        }
        if let Some(m) = max_items {
            val.max_items = m;
        }

        Ok(val)
    }
}

pub fn get_cur_cmd() -> Result<TCmd> {
    let vector: Vec<String> = env::args()
        .skip(1)
        .filter(|f| !f.starts_with("--"))
        .collect();

    let mut args = vector.iter();

    // TODO: handle other commands here

    if args.len() >= 3 {
        bail!("Only two positional arguments are allowed.")
    }

    if let Some(arg1) = args.next() {
        static SARGERROR: &str = "Second argument must be either a duration or a multiplier.";

        let cmd = if let Ok(x) = TDuration::parse_str(arg1) {
            if let Some(arg2) = args.next() {
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
            if let Some(arg2) = args.next() {
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
