use crate::cli::args::Command;
use anyhow::Result;

pub mod budget;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self) -> Result<()> {
        match self {
            Command::Budget(budget_cmd) => budget_cmd.run(),
            Command::Trim(trim_cmd) => trim_cmd.run(),
            Command::Yt(yt_cmd) => yt_cmd.run(),
        }
    }
}
