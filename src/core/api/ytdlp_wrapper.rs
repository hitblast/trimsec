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
struct TYtdlpVideo {
    id: String,
    duration: f64,
}

#[derive(Deserialize)]
struct TYtdlpPlaylist {
    entries: Vec<TYtdlpVideo>,
}

impl YtInnerYtdlpWrapper {
    pub fn new() -> Result<Self, TYtClientInitError> {
        let result = which("yt-dlp").map_err(|e| TYtClientInitError::Ytdlp(e.to_string()));

        Ok(Self { path: result? })
    }

    fn execute_command(&self, args: &[String]) -> Result<String, TYtClientError> {
        let mut cmd = std::process::Command::new(&self.path);
        cmd.args(args);

        // global flags which must be set for all commands
        // otherwise yt-dlp output might get messy
        cmd.args(["--no-warnings", "--skip-download", "--dump-single-json"]);

        let output = cmd
            .output()
            .map_err(|e| TYtClientError::CmdSpawnError(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let err_msg = if stderr.trim().is_empty() {
                "Unknown error (no stderr output)".to_string()
            } else {
                stderr.trim().to_string()
            };

            return Err(TYtClientError::ErroneousCmdOutput(err_msg));
        }

        String::from_utf8(output.stdout).map_err(|e| TYtClientError::BufReaderError(e.to_string()))
    }

    fn execute_parse<T: DeserializeOwned>(&self, args: &[String]) -> Result<T, TYtClientError> {
        let output = self.execute_command(args)?;

        serde_json::from_str(&output)
            .map_err(|e| TYtClientError::DeserializationError(e.to_string()))
    }

    /// deprecated: use `execute_parse` instead for single-pass operations
    /// (preferred since `yt-dlp --print` is no longer used internally)
    #[deprecated]
    fn _execute_parse_vec<T: DeserializeOwned>(
        &self,
        args: &[String],
    ) -> Result<Vec<T>, TYtClientError> {
        let output = self.execute_command(args)?;

        output
            .lines()
            .map(|line| {
                serde_json::from_str(line)
                    .map_err(|e| TYtClientError::DeserializationError(e.to_string()))
            })
            .collect()
    }
}

impl YtInnerYtdlpWrapper {
    fn fetch_playlist(
        &self,
        mut args: Vec<String>,
        max_items: usize,
    ) -> Result<TYtdlpPlaylist, crate::errors::TYtClientError> {
        //
        // using --flat-playlist significantly speeds up command execution
        // for large playlists, then we can just deserialize the "entries" field for all the videoss
        args.push("--flat-playlist".to_string());

        if max_items != 0 {
            args.push("--playlist-end".to_string());
            args.push(max_items.to_string());
        }

        let output: TYtdlpPlaylist = self.execute_parse(&args)?;
        Ok(output)
    }
}

impl YtClientCommon for YtInnerYtdlpWrapper {
    fn fetch_duration_from_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<crate::core::time::TDuration, crate::errors::TYtClientError> {
        let vec = vec![id.construct_url()];

        if !id.is_playlist() {
            let output = self.execute_parse::<TYtdlpVideo>(&vec)?;
            return Ok(TDuration::from(output.duration));
        }

        let output = self.fetch_playlist(vec, max_items)?;

        Ok(output
            .entries
            .iter()
            .map(|e| TDuration::from(e.duration))
            .sum())
    }

    fn expand_id(
        &self,
        id: &TYoutubeId,
        max_items: usize,
    ) -> Result<Vec<TYoutubeId>, crate::errors::TYtClientError> {
        if !id.is_playlist() {
            return Ok(vec![id.clone()]);
        }

        let output = self.fetch_playlist(vec![id.construct_url()], max_items)?;

        Ok(output
            .entries
            .into_iter()
            .map(|e| TYoutubeId::new(e.id, false))
            .collect())
    }

    #[allow(unused)]
    fn fetch_video_items(
        &self,
        ids: &[TYoutubeId],
    ) -> Result<Vec<crate::core::api::types::YtVideo>, crate::errors::TYtClientError> {
        todo!()
    }
}
