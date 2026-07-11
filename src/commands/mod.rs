use crate::cli::args::Command;
use anyhow::Result;

pub mod fitcheck;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self, no_clip: bool) -> Result<()> {
        match self {
            Command::Fitcheck(fitcheck_cmd) => fitcheck_cmd.run(no_clip),
            Command::Trim(trim_cmd) => trim_cmd.run(),
            Command::Yt(yt_cmd) => yt_cmd.run(no_clip),
        }
    }
}
