use crate::{
    args::{ColorMode, TKeywordArgs},
    core::{api::YtClient, config::Config},
    style::Style,
};
use anyhow::{Result, anyhow};

pub struct Ctx {
    pub style: Style,
    pub kwargs: Option<TKeywordArgs>,
    config: Option<Config>,
    api_client: Option<YtClient>,
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

    pub fn client(&mut self) -> Result<&YtClient> {
        match self.api_client {
            Some(ref c) => Ok(c),
            None => {
                let client = YtClient::new(self.config()?).map_err(|e| anyhow::anyhow!(e))?;
                let inner = self.api_client.insert(client);
                Ok(inner)
            }
        }
    }
}
