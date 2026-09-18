use crate::{
    args::{ColorMode, TKeywordArgs},
    core::config::Config,
    style::Style,
};
use anyhow::{Result, anyhow};

pub mod fit;
pub mod key_set;
pub mod key_show;
pub mod list;
pub mod path;
pub mod trim;

pub struct Ctx<'a> {
    pub style: Style,
    pub kwargs: &'a TKeywordArgs,
    config: Option<Config>,
}

impl<'a> Ctx<'a> {
    #[must_use]
    pub fn new(color: &ColorMode, kwargs: &'a TKeywordArgs) -> Self {
        Ctx {
            style: Style::determine(color),
            kwargs,
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
