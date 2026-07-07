use crate::cli::args::Command;
use anyhow::Result;

pub mod fit;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Fit(fit_cmd) => fit_cmd.run(),
            Command::Trim(trim_cmd) => trim_cmd.run(),
            Command::Yt(yt_cmd) => yt_cmd.run(),
        }
    }
}
