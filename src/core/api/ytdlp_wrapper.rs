use std::io::{BufRead, BufReader, Read};

use crate::{
    core::{api::types::YtClientCommon, time::TDuration, youtils::TYoutubeId},
    errors::{TYtClientError, TYtClientInitError},
};
use serde::{Deserialize, de::DeserializeOwned};
use which::which;

pub struct YtInnerYtdlpWrapper {
    path: std::path::PathBuf,
}

#[derive(Deserialize)]
struct TYtdlpDuration {
    duration: f64,
}

const YT_DLP_ERROR_MSG_UNSUPPORTED: &str = "yt-dlp can only be used to fetch single-videos. For playlists, please use the YouTube Data API by enabling it via `ts key set`.";

impl YtInnerYtdlpWrapper {
    pub fn new() -> Result<Self, TYtClientInitError> {
        let result = which("yt-dlp").map_err(|e| TYtClientInitError::Ytdlp(e.to_string()));

        Ok(Self { path: result? })
    }

    pub fn execute<T: DeserializeOwned>(&self, args: &[String]) -> Result<Vec<T>, TYtClientError> {
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(args);
        cmd.args(&["--no-warnings", "--skip-download"]);

        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| TYtClientError::CmdSpawnError(e.to_string()))?;

        let stdout = child.stdout.take().ok_or(TYtClientError::StdoutTakeError)?;
        let mut stderr = child.stderr.take().ok_or(TYtClientError::StderrTakeError)?;

        let mut vec: Vec<T> = Vec::new();

        for line in BufReader::new(stdout).lines() {
            let line = line.map_err(|e| TYtClientError::BufReaderError(e.to_string()))?;
            let data: T = serde_json::from_str(&line)
                .map_err(|e| TYtClientError::DeserializationError(e.to_string()))?;
            vec.push(data);
        }

        let status = child
            .wait()
            .map_err(|e| TYtClientError::ErroneousCmdOutput(e.to_string()))?;

        if !status.success() {
            let mut err_msg = String::new();
            stderr
                .read_to_string(&mut err_msg)
                .map_err(|e| TYtClientError::BufReaderError(e.to_string()))?;

            let err_msg = if err_msg.trim().is_empty() {
                "Unknown error (no stderr output)".to_string()
            } else {
                err_msg.trim().to_string()
            };

            return Err(TYtClientError::ErroneousCmdOutput(err_msg));
        }

        Ok(vec)
    }
}

macro_rules! unsupported {
    () => {
        return Err(crate::errors::TYtClientError::UnsupportedError(
            YT_DLP_ERROR_MSG_UNSUPPORTED,
        ))
    };
}
impl YtClientCommon for YtInnerYtdlpWrapper {
    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<crate::core::time::TDuration, crate::errors::TYtClientError> {
        let mut vec = vec![
            "--print".to_string(),
            "%(.{duration})j".to_string(),
            id.construct_url(),
        ];

        if id.is_playlist() && max_items != 0 {
            vec.push("--playlist-end".to_string());
            vec.push(max_items.to_string());
        }

        let output: Vec<TYtdlpDuration> = self.execute(&vec)?;
        let duration: TDuration = output.iter().map(|f| TDuration::from(f.duration)).sum();

        Ok(TDuration::from(duration))
    }

    #[allow(unused)]
    fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeId>, crate::errors::TYtClientError> {
        unsupported!()
    }

    #[allow(unused)]
    fn fetch_video_items(
        &self,
        ids: &[TYoutubeId],
    ) -> Result<Vec<crate::core::api::types::YtVideo>, crate::errors::TYtClientError> {
        unsupported!()
    }
}
