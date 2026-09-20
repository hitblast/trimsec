use crate::core::{context::Ctx, youtils::TYoutubeId};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct KeySetCmd {
    /// The API key to set.
    api_key: String,

    /// Skip API key authenticity check.
    #[arg(short, long)]
    no_check: bool,
}

impl KeySetCmd {
    pub fn run(self, ctx: &mut Ctx) -> Result<()> {
        if ctx.config()?.api_key().is_some_and(|f| f == self.api_key) {
            println!("Key is already installed.");
            return Ok(());
        }

        if !self.no_check {
            println!("Testing key... (use --no-check to skip)");

            let id = TYoutubeId::new("dQw4w9WgXcQ", false);

            if ctx.client()?.fetch_duration_from_id(&id, 1).is_err() {
                bail!(
                    "{}Invalid API key passed!{}",
                    ctx.style.red(),
                    ctx.style.reset()
                )
            }
        }

        ctx.config()?
            .update_write_key(self.api_key)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        println!("Key added successfully.");

        Ok(())
    }
}
