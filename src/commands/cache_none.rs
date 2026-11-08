use crate::core::context::Ctx;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct CacheNoneCmd;

impl CacheNoneCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        ctx.client().and_then(|f| {
            match f {
                crate::core::youtube::YtClient::ApiBased(yt_inner_data_api_client) => {
                    if let Some(manager) = yt_inner_data_api_client.cache_manager() {
                        smol::block_on(manager.clear())
                            .map_err(|e| anyhow::anyhow!("Error clearing cache: {e}"))?;
                    }
                }
                crate::core::youtube::YtClient::Ytdlp(_) => {
                    println!("Cache not supported in yt-dlp-based clients.")
                }
            }

            Ok(())
        })
    }
}
