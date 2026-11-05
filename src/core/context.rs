use crate::{
    args::{ColorMode, TKeywordArgs},
    core::{
        api::{YtInnerApiClient, types::YtInnerClient},
        config::Config,
        youtils::decide_youtube_key,
    },
    style::Style,
};
use anyhow::{Result, anyhow};

pub struct Ctx {
    pub style: Style,
    pub kwargs: Option<TKeywordArgs>,
    config: Option<Config>,
    api_client: Option<YtClient>,
}

pub enum YtClient {
    Ytdlp(),
    ApiBased(YtInnerApiClient),
}

impl YtClient {
    pub fn inner(&self) -> Result<&impl YtInnerClient> {
        match self {
            YtClient::ApiBased(c) => Ok(c),
            _ => unimplemented!(),
        }
    }
}

impl Ctx {
    #[must_use]
    pub fn new(kwargs: TKeywordArgs) -> Self {
        Ctx {
            style: Style::new(kwargs.color()),
            kwargs: Some(kwargs),
            config: None,
            api_client: None,
        }
    }

    #[must_use]
    pub fn new_bare() -> Self {
        Ctx {
            style: Style::new(&ColorMode::Auto),
            kwargs: None,
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

    pub fn client(&mut self) -> Result<&impl YtInnerClient> {
        match self.api_client {
            Some(ref c) => Ok(c.inner()?),
            None => {
                let cfg = self.config()?;
                let key = decide_youtube_key(cfg);

                match key {
                    Some(key) => {
                        let client = YtInnerApiClient::new(key).map_err(|e| anyhow::anyhow!(e))?;
                        let ins = self.api_client.insert(YtClient::ApiBased(client)).inner()?;
                        Ok(ins)
                    }
                    None => Err(anyhow!("no youtube key")),
                }
            }
        }
    }
}
