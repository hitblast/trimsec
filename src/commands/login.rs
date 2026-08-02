use crate::{cli::flags::Flags, commands::Runnable, core::style::Style};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct LoginCmd;

impl Runnable for LoginCmd {
    fn run(self, flags: &Flags, _: &Style) -> Result<()> {
        Ok(())
    }
}
