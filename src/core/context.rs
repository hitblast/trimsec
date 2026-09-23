use crate::{
    args::TKeywordArgs,
    core::{api::ApiClient, config::Config, youtils::decide_youtube_key},
    style::Style,
};
use anyhow::{Result, anyhow};

pub struct Ctx {
    pub style: Style,
    pub kwargs: TKeywordArgs,
    config: Option<Config>,
    api_client: Option<ApiClient>,
}

impl Ctx {
    #[must_use]
    pub fn new(kwargs: TKeywordArgs) -> Self {
        Ctx {
            style: Style::new(kwargs.color()),
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
                let key = decide_youtube_key(cfg)?;

                let client = ApiClient::new(key);
                let ins = self.api_client.insert(client);

                Ok(ins)
            }
        }
    }
}
