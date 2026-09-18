use crate::{args::ColorMode, core::config::Config, style::Style};
use anyhow::{Result, anyhow};

pub mod fit;
pub mod key_set;
pub mod key_show;
pub mod list;
pub mod trim;

pub struct Ctx {
    pub style: Style,
    config: Option<Config>,
}

impl Ctx {
    #[must_use]
    pub fn new(color: &ColorMode) -> Self {
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
