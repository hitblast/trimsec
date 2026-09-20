use crate::{
    args::{ColorMode, TKeywordArgs},
    core::{api::ApiClient, config::Config, youtils::decide_youtube_key},
    style::Style,
};
use anyhow::{Result, anyhow};

pub struct Ctx<'a> {
    pub style: Style,
    pub kwargs: &'a TKeywordArgs,
    config: Option<Config>,
    api_client: Option<ApiClient>,
}

impl<'a> Ctx<'a> {
    #[must_use]
    pub fn new(color: &ColorMode, kwargs: &'a TKeywordArgs) -> Self {
        Ctx {
            style: Style::determine(color),
            kwargs,
            config: None,
            api_client: None,
        }
    }

    pub fn config(&mut self) -> Result<&mut Config> {
        match self.config {
            Some(ref mut c) => Ok(c),
            None => {
                let cfg = Config::load().map_err(|e| anyhow!("config load failure: {e}"))?;
                Ok(self.config.insert(cfg))
            }
        }
    }

    pub fn client(&mut self) -> Result<&ApiClient> {
        match self.api_client {
            Some(ref c) => Ok(c),
            None => {
                let cfg = self.config()?;
                let key = decide_youtube_key(&cfg)?;

                let client = ApiClient::new(key);
                let ins = self.api_client.insert(client);

                Ok(ins)
            }
        }
    }
}
