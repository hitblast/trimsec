use crate::core::context::Ctx;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct KeyUnsetCmd;

impl KeyUnsetCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        let cfg = ctx.config()?;
        if cfg.api_key().is_none() {
            return Ok(());
        }

        cfg.unset_key()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        println!("Key removed.");

        Ok(())
    }
}
