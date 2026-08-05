use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{api::ApiClientManager, config::Config, style::Style, youtils::YoutubeId},
};
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

impl Runnable for KeySetCmd {
    fn run(self, _flags: &Flags, style: &Style) -> Result<()> {
        let mut config = Config::load().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if !self.no_check {
            println!("Testing key... (use --no-check to skip)");
            let client = ApiClientManager::new(&self.api_key);

            let id = YoutubeId {
                id: "dQw4w9WgXcQ".to_string(),
                is_playlist: false,
            };
            if let Err(_) = client.fetch_duration_from_id(&id, 1) {
                bail!("{}Invalid API key passed!{}", style.red(), style.reset())
            }
        }

        config
            .update_write_key(self.api_key)
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        println!("Key added successfully.");

        Ok(())
    }
}
