use crate::{
    cli::args::{ColorMode, Command},
    commands::style::Style,
    core::config::Config,
};
use anyhow::{Result, anyhow};

pub mod fit;
pub mod key_set;
pub mod key_show;
pub mod list;
pub mod path;
pub mod trim;

mod style;

pub struct Ctx {
    pub style: Style,
    config: Option<Config>,
}

impl Ctx {
    #[must_use]
    pub fn new(color: ColorMode) -> Self {
        Ctx {
            style: Style::determine(color),
            config: None,
        }
    }

    pub fn config(&mut self) -> Result<&mut Config> {
        if self.config.is_none() {
            self.config = Some(Config::load().map_err(|e| anyhow!("config load failure: {e}"))?);
        }
        #[allow(clippy::unwrap_used)]
        Ok(self.config.as_mut().unwrap())
    }
}

impl Command {
    pub fn run(self, color: ColorMode) -> Result<()> {
        let mut ctx = Ctx::new(color);

        match self {
            Command::Fit(fits_cmd) => fits_cmd.run(&mut ctx),
            Command::List(list_cmd) => list_cmd.run(&mut ctx),
            Command::Key { command } => match command {
                crate::cli::args::KeySubcmd::Show(key_show_cmd) => key_show_cmd.run(&mut ctx),
                crate::cli::args::KeySubcmd::Set(key_set_cmd) => key_set_cmd.run(&mut ctx),
            },
            Command::Path(path_cmd) => path_cmd.run(&mut ctx),
            Command::Trim(trim_cmd) => trim_cmd.run(&mut ctx),
        }
    }
}

pub trait Runnable {
    fn run(self, ctx: &mut Ctx) -> Result<()>;
}
