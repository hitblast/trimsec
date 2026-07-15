use std::fs;

use serde::Deserialize;

use crate::{core::utils::get_rc_filepath, errors::TConfigError};

#[derive(Debug, Deserialize)]
pub struct Config {
    api_key: String,
}

impl Config {
    pub fn load() -> Result<Self, TConfigError> {
        match get_rc_filepath() {
            Ok(p) => {
                let data = fs::read_to_string(&p);

                match data {
                    Ok(data) => match toml::from_str::<Self>(&data) {
                        Ok(cfg) => Ok(cfg),
                        Err(_) => Err(TConfigError::ParseFailed(p)),
                    },
                    Err(e) => return Err(TConfigError::PathReadFailure(e.to_string())),
                }
            }
            Err(e) => return Err(TConfigError::NonexistentPath(e.to_string())),
        }
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_ref()
    }
}
