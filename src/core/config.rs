use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{core::utils::get_config_path, errors::TConfigError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    api_key: Option<String>,
    #[serde(skip)]
    path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, TConfigError> {
        let Ok(p) = get_config_path() else {
            return Err(TConfigError::UnavailableConfigPath);
        };

        if !p.try_exists().unwrap_or(false) {
            let parent = p.parent().ok_or(TConfigError::InvalidParentPath)?;

            if !parent.try_exists().unwrap_or(false) {
                fs::create_dir_all(parent)
                    .map_err(|e| TConfigError::DirectoryCreationFailure(e.to_string()))?;
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

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
