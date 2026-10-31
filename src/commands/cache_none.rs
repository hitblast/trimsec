use crate::core::context::Ctx;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct CacheNoneCmd;

impl CacheNoneCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        ctx.client().and_then(|f| {
            match f.cache_path() {
                Some(p) => {
                    println!("path: {}", p.display(),);
                }
                None => println!("cache path not found"),
            }

            Ok(())
        })
    }
}
