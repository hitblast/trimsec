use std::fs;

use crate::core::context::Ctx;
use anyhow::{Result, bail};
use clap::Args;
use filesize::PathExt;

#[derive(Debug, Default, Args)]
pub struct CacheCleanCmd;

impl CacheCleanCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        ctx.client().and_then(|f| {
            let Some(path) = f.cache_path() else {
                return Ok(());
            };

            if !path.exists() || !f.is_preexisting_cache_file() {
                println!("Cache does not exist.");
                return Ok(());
            }

            let mut size = path.size_on_disk()? / 1024;

            if let Err(e) = fs::remove_file(path) {
                bail!("Error while removing cache: {e}")
            }

            if size > 1024 {
                size = size / 1024;
                println!("Cleaned {size} MB.");
            } else {
                println!("Cleaned {size} KB.");
            }

            Ok(())
        })
    }
}
