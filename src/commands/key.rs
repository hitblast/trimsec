use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{config::Config, style::Style},
};
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct KeyCmd {
    /// The API key to set.
    api_key: String,
}

impl Runnable for KeyCmd {
    fn run(self, _flags: &Flags, _style: &Style) -> Result<()> {
        let mut config = Config::load().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        config
            .update_write_key(self.api_key)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
