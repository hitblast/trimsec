use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{core::time::TDuration, errors::TConfigError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    api_key: Option<String>,
    options: Option<ConfigOptions>,
    #[serde(skip)]
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigOptions {
    default_fit_budget: Option<TDuration>,
}

impl ConfigOptions {
    #[must_use]
    pub fn default_fit_budget(&self) -> Option<&TDuration> {
        self.default_fit_budget.as_ref()
    }
}

fn get_config_path() -> Result<PathBuf, TConfigError> {
    let home = dirs::home_dir();

    if let Some(h) = home {
        Ok(h.join(".trimsecrc").to_path_buf())
    } else {
        Err(TConfigError::UnavailableConfigPath)
    }
}

impl Config {
    pub fn load() -> Result<Self, TConfigError> {
        let p = get_config_path()?;

        if !p.try_exists().unwrap_or(false) {
            let parent = p.parent().ok_or(TConfigError::InvalidParentPath)?;

            if !parent.try_exists().unwrap_or(false) {
                return Err(TConfigError::HomeNotFound);
            }

            fs::write(&p, "").map_err(|e| TConfigError::SaveFailed(e.to_string()))?;
        }

        let data = fs::read_to_string(&p);

        match data {
            Ok(data) => match toml::from_str::<Self>(&data) {
                Ok(mut cfg) => {
                    cfg.path = p;
                    Ok(cfg)
                }
                Err(_) => Err(TConfigError::DeserializingFailed(p)),
            },
            Err(e) => Err(TConfigError::ConfigReadFailure(e.to_string())),
        }
    }

    pub fn update_write_key(&mut self, new_key: String) -> Result<(), TConfigError> {
        self.api_key = Some(new_key);
        self.save()?;
        Ok(())
    }

    fn save(&self) -> Result<(), TConfigError> {
        let data =
            toml::to_string(&self).map_err(|e| TConfigError::SerializingFailed(e.to_string()))?;
        fs::write(&self.path, data).map_err(|e| TConfigError::SaveFailed(e.to_string()))?;

        Ok(())
    }

    #[must_use]
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    #[must_use]
    pub fn options(&self) -> Option<&ConfigOptions> {
        self.options.as_ref()
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
