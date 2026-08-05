use crate::{commands::Runnable, core::config::Config};
use clap::Args;

#[derive(Args, Debug)]
pub struct KeyShowCmd;

impl Runnable for KeyShowCmd {
    fn run(
        self,
        _flags: &crate::cli::flags::Flags,
        _style: &crate::core::style::Style,
    ) -> anyhow::Result<()> {
        let config: Config = Config::load().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        println!("{}", config.api_key().unwrap_or("not set"));
        Ok(())
    }
}
