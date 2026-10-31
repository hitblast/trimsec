use crate::core::context::Ctx;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct CacheCleanCmd;

impl CacheCleanCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        ctx.client().and_then(|f| {
            if let Some(manager) = f.cache_manager() {
                smol::block_on(manager.clear())
                    .map_err(|e| anyhow::anyhow!("Error clearing cache: {e}"))?;
            }

            Ok(())
        })
    }
}
