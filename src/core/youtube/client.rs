use enum_dispatch::enum_dispatch;

use crate::{
    core::{
        config::Config,
        time::TDuration,
        youtube::{
            api_client::YtInnerDataApiClient,
            traits::YtClientCommon,
            youtils::{TYoutubeId, TYoutubeVideo, decide_youtube_key},
            ytdlp_wrapper::YtInnerYtdlpWrapper,
        },
    },
    errors::{TYtClientError, TYtClientInitError},
};

#[enum_dispatch(YtClientCommon)]
pub enum YtClient {
    Ytdlp(YtInnerYtdlpWrapper),
    ApiBased(YtInnerDataApiClient),
}

impl YtClient {
    pub fn new(cfg: &Config) -> Result<Self, TYtClientInitError> {
        let key = decide_youtube_key(cfg);

        match key {
            Some(key) => {
                let client = YtInnerDataApiClient::new(key)?;
                Ok(Self::ApiBased(client))
            }
            None => {
                let client = YtInnerYtdlpWrapper::new()?;
                Ok(Self::Ytdlp(client))
            }
        }
    }
}
