use std::path::PathBuf;

use anyhow::{Result, bail};

pub fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir();

    if let Some(h) = home {
        Ok(h.join(".trimsecrc").to_path_buf())
    } else {
        bail!("Could not determine HOME directory.")
    }
}
