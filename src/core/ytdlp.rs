use std::process::Output;

use crate::{
    core::{
        api::types::{YtInnerClient, YtVideo},
        youtils::TYoutubeId,
    },
    errors::YtInnerClientInitError,
};
use which::which;

pub struct Ytdlp {
    path: std::path::PathBuf,
}

macro_rules! execute {
    ($self:expr, $($args:tt)*) => {
        $self.execute(&format!($($args)*))
    };
}

impl Ytdlp {
    pub fn execute(&self, subcmd: &str) -> Result<Output, anyhow::Error> {
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(subcmd.split_whitespace());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let output = cmd.output()?;
        if !output.status.success() {
            anyhow::bail!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(output)
    }

    pub fn fetch_duration(&self, id: &TYoutubeId) -> Result<super::time::TDuration, anyhow::Error> {
        let output = execute!(self, "--print duration {}", id)?;

        let duration = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<f64>()?;

        Ok(super::time::TDuration::from(duration))
    }
}

impl YtInnerClient for Ytdlp {
    fn new(_: impl ToString) -> Result<Self, YtInnerClientInitError>
    where
        Self: Sized,
    {
        let result = which("yt-dlp").map_err(|e| YtInnerClientInitError::Ytdlp(e.to_string()));

        Ok(Self { path: result? })
    }

    fn expand_id(
        &self,
        id: &super::youtils::TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<super::youtils::TYoutubeId>, crate::errors::TYoutubeError> {
        todo!()
    }

    fn fetch_video_items(
        &self,
        ids: &[super::youtils::TYoutubeId],
    ) -> Result<Vec<YtVideo>, crate::errors::TYoutubeError> {
        todo!()
    }

    fn fetch_duration_from_id(
        &self,
        id: &super::youtils::TYoutubeId,
        max_items: usize,
    ) -> Result<super::time::TDuration, crate::errors::TYoutubeError> {
        todo!()
    }

    fn cache_path(&self) -> Option<&std::path::Path> {
        todo!()
    }

    fn cache_manager(&self) -> Option<&http_cache_ureq::RedbManager> {
        todo!()
    }
}
