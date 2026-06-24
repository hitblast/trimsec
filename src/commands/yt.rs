use clap::Args;

use crate::commands::Runnable;
use anyhow::Result;

#[derive(Debug, Default, Args)]
pub struct YtCmd;

impl Runnable for YtCmd {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
