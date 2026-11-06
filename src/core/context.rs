use crate::{
    args::{ColorMode, TKeywordArgs},
    core::{
        api::{
            YtInnerDataApiClient,
            types::{YtClientCommon, YtVideo},
        },
        config::Config,
        time::TDuration,
        youtils::{TYoutubeId, decide_youtube_key},
        ytdlp::YtInnerYtdlpWrapper,
    },
    errors::TYtClientError,
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
    Ytdlp(YtInnerYtdlpWrapper),
    ApiBased(YtInnerDataApiClient),
}

impl YtClientCommon for YtClient {
    fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeId>, TYtClientError> {
        match self {
            YtClient::Ytdlp(yt_inner_ytdlp_wrapper) => {
                yt_inner_ytdlp_wrapper.expand_id(id, max_items)
            }
            YtClient::ApiBased(yt_inner_api_client) => yt_inner_api_client.expand_id(id, max_items),
        }
    }

    fn fetch_video_items(&self, ids: &[TYoutubeId]) -> Result<Vec<YtVideo>, TYtClientError> {
        match self {
            YtClient::Ytdlp(yt_inner_ytdlp_wrapper) => {
                yt_inner_ytdlp_wrapper.fetch_video_items(ids)
            }
            YtClient::ApiBased(yt_inner_api_client) => yt_inner_api_client.fetch_video_items(ids),
        }
    }

    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<TDuration, TYtClientError> {
        match self {
            YtClient::Ytdlp(yt_inner_ytdlp_wrapper) => {
                yt_inner_ytdlp_wrapper.fetch_duration_from_id(id, max_items)
            }
            YtClient::ApiBased(yt_inner_api_client) => {
                yt_inner_api_client.fetch_duration_from_id(id, max_items)
            }
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

    pub fn client(&mut self) -> Result<&YtClient> {
        match self.api_client {
            Some(ref c) => Ok(c),
            None => {
                let cfg = self.config()?;
                let key = decide_youtube_key(cfg);

                match key {
                    Some(key) => {
                        let client =
                            YtInnerDataApiClient::new(key).map_err(|e| anyhow::anyhow!(e))?;
                        let ins = self.api_client.insert(YtClient::ApiBased(client));
                        Ok(ins)
                    }
                    None => {
                        let client = YtInnerYtdlpWrapper::new().map_err(|e| anyhow::anyhow!(e))?;
                        let ins = self.api_client.insert(YtClient::Ytdlp(client));
                        Ok(ins)
                    }
                }
            }
        }
    }
}
