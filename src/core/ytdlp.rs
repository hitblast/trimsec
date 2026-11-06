use std::io::{BufRead, BufReader, Read};

use crate::{
    core::{
        api::types::{YtInnerClient, YtVideo},
        time::TDuration,
        youtils::TYoutubeId,
    },
    errors::{TYtClientError, TYtInnerClientInitError},
};
use anyhow::bail;
use serde::{Deserialize, de::DeserializeOwned};
use which::which;

pub struct YtdlpWrapper {
    path: std::path::PathBuf,
}

#[derive(Deserialize)]
pub struct TYtdlpDuration {
    duration: f64,
}

impl YtdlpWrapper {
    pub fn execute<T: DeserializeOwned>(&self, args: &[String]) -> Result<Vec<T>, anyhow::Error> {
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(args);
        cmd.args(&["--no-warnings", "--skip-download"]);

        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let mut stderr = child
            .stderr
            .take()
            .ok_or(anyhow::anyhow!("could not take stderr from child"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or(anyhow::anyhow!("could not take stdout from child"))?;

        let mut vec: Vec<T> = Vec::new();

        for line in BufReader::new(stdout).lines() {
            let line = line?;
            let data: T = serde_json::from_str(&line)?;
            vec.push(data);
        }

        let status = child.wait()?;

        if !status.success() {
            let mut err_msg = String::new();
            stderr.read_to_string(&mut err_msg)?;

            let err_msg = if err_msg.trim().is_empty() {
                "Unknown error (no stderr output)".to_string()
            } else {
                err_msg.trim().to_string()
            };

            bail!("Command execution failed: {}", err_msg);
        }

        Ok(vec)
    }

    pub fn fetch_duration(
        &self,
        id: &TYoutubeId,
        playlist_end: usize,
    ) -> Result<super::time::TDuration, anyhow::Error> {
        let mut vec = vec![
            "--print".to_string(),
            "%(.{duration})j".to_string(),
            id.construct_url(),
        ];

        if id.is_playlist() && playlist_end != 0 {
            vec.push(format!("--playlist-end {playlist_end}"));
        }

        let output: Vec<TYtdlpDuration> = self.execute(&vec)?;
        let duration: TDuration = output.iter().map(|f| TDuration::from(f.duration)).sum();

        Ok(TDuration::from(duration))
    }
}

const YT_DLP_ERROR_MSG_UNSUPPORTED: &str = "yt-dlp can only be used to fetch single-videos. For playlists, please use the YouTube Data API by enabling it via `ts key set`.";

impl YtInnerClient for YtdlpWrapper {
    fn new(_: impl ToString) -> Result<Self, TYtInnerClientInitError>
    where
        Self: Sized,
    {
        let result = which("yt-dlp").map_err(|e| TYtInnerClientInitError::Ytdlp(e.to_string()));

        Ok(Self { path: result? })
    }

    // unsupported: ytdlp performance regression
    fn expand_id(
        &self,
        _: &super::youtils::TYoutubeId,
        _: usize,
    ) -> Result<Vec<super::youtils::TYoutubeId>, crate::errors::TYtClientError> {
        Err(TYtClientError::UnsupportedError(
            YT_DLP_ERROR_MSG_UNSUPPORTED.to_string(),
        ))
    }

    // unsupported: ytdlp performance regression
    fn fetch_video_items(
        &self,
        _: &[super::youtils::TYoutubeId],
    ) -> Result<Vec<YtVideo>, crate::errors::TYtClientError> {
        Err(TYtClientError::UnsupportedError(
            YT_DLP_ERROR_MSG_UNSUPPORTED.to_string(),
        ))
    }

    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<super::time::TDuration, crate::errors::TYtClientError> {
        self.fetch_duration(id, max_items)
            .map_err(|e| crate::errors::TYtClientError::UreqError(e.to_string()))
    }

    fn cache_path(&self) -> Option<&std::path::Path> {
        None
    }

    fn cache_manager(&self) -> Option<&http_cache_ureq::RedbManager> {
        None
    }
}
