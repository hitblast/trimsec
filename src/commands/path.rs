use crate::{commands::Runnable, core::utils::get_config_path};
use clap::Args;

#[derive(Debug, Args)]
pub struct PathCmd;

impl Runnable for PathCmd {
    fn run(
        self,
        _: &crate::cli::flags::Flags,
        _: &crate::core::style::Style,
    ) -> anyhow::Result<()> {
        let cfg_path = get_config_path()?;

        println!("{}", cfg_path.display());
        Ok(())
    }
}
