use crate::core::context::Ctx;
use anyhow::Result;
use clap::Args;
use filesize::PathExt;

#[derive(Debug, Default, Args)]
pub struct CacheNoneCmd;

impl CacheNoneCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        ctx.client().and_then(|f| {
            match f.cache_path() {
                Some(p) => {
                    println!(
                        r#"
new cache? {}
path: {}
size: {} KB
                    "#,
                        if !f.is_preexisting_cache_file() {
                            "yes"
                        } else {
                            "no"
                        },
                        p.display(),
                        p.size_on_disk()? / 1024,
                    );
                }
                None => println!("cache path not found"),
            }

            Ok(())
        })
    }
}
