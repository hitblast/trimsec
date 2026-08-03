use crate::{
    cli::flags::Flags,
    commands::Runnable,
    core::{api::ApiClientManager, config::Config, style::Style, youtils::YoutubeId},
};
use anyhow::{Result, bail};
use clap::Args;

#[derive(Debug, Default, Args)]
pub struct KeyCmd {
    /// Show the current API key.
    #[arg(long)]
    show: bool,

    /// The API key to set.
    #[arg(required_unless_present = "show")]
    api_key: Option<String>,

    /// Do not check the authenticity of the key.
    #[arg(short, long)]
    no_check: bool,
}

impl Runnable for KeyCmd {
    fn run(self, _flags: &Flags, style: &Style) -> Result<()> {
        let mut config = Config::load().map_err(|e| anyhow::anyhow!(e.to_string()))?;

        if self.show {
            return Ok(println!("{}", config.api_key().unwrap_or("No key set.")));
        }

        if let Some(k) = self.api_key {
            if !self.no_check {
                println!("Testing key... (use --no-check to skip)");
                let client = ApiClientManager::new(&k);

                let id = YoutubeId {
                    id: "dQw4w9WgXcQ".to_string(),
                    is_playlist: false,
                };
                if let Err(_) = client.fetch_duration_from_id(&id, 1) {
                    bail!("{}Invalid API key passed!{}", style.red, style.reset)
                }
            }

            config
                .update_write_key(k)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            println!("Key added successfully.");
        }

        Ok(())
    }
}
