use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum TConfigError {
    InvalidParentPath,
    UnavailableConfigPath,
    HomeNotFound,
    ConfigReadFailure(String),
    DeserializingFailed(PathBuf),
    SerializingFailed(String),
    SaveFailed(String),
}

impl Display for TConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TConfigError::InvalidParentPath => write!(f, "invalid parent path for config file"),
            TConfigError::UnavailableConfigPath => {
                write!(f, "could not fetch absolute config path")
            }
            TConfigError::HomeNotFound => {
                write!(f, "home  directory not found")
            }
            TConfigError::ConfigReadFailure(e) => write!(f, "failed to read config file: {e}"),
            TConfigError::DeserializingFailed(p) => {
                write!(f, "could not parse file at path: {p:?}")
            }
            TConfigError::SerializingFailed(e) => write!(f, "could not serialize config: {e}"),
            TConfigError::SaveFailed(p) => write!(f, "could not save file to path: {p:?}"),
        }
    }
}

#[derive(Debug)]
pub enum ParseTDurationError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InvalidMultiplierFormat,
    EmptyDurationString,
    MultiplierOutOfRange,
}

impl Display for ParseTDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            Self::InvalidTimeUnit => {
                "specify duration in seconds (s), minutes (m), hours (h), or days (d)"
            }
            Self::InvalidDurationFormat => "invalid duration format!",
            Self::NegativeDuration => "duration must be a positive value.",
            Self::InvalidMultiplierFormat => "multiplier must be a positive float.",
            Self::MultiplierOutOfRange => "multiplier must be greater than 1x and less than 100x.",
            Self::EmptyDurationString => "empty duration-string passed.",
        };
        write!(f, "{wr}")
    }
}

#[derive(Debug)]
pub enum TYoutubeError {
    UreqError(ureq::Error),
    RequestUriParseError(String),
    InvalidResponseBody,
    ItemNotFound,
    InvalidPlaylist(String),
    InvalidMaxSize((usize, usize)),
}

impl Display for TYoutubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            TYoutubeError::UreqError(e) => format!("error performing request: {e}."),
            TYoutubeError::RequestUriParseError(e) => format!("error parsing request Uri: {e}"),
            TYoutubeError::ItemNotFound => {
                "given YouTube video item was not found in API response.".to_string()
            }
            TYoutubeError::InvalidPlaylist(id) => format!("invalid playlist: {id}"),
            TYoutubeError::InvalidMaxSize((given, max)) => {
                format!("max items ({given}) is larger than the length of the playlist ({max}).")
            }
            TYoutubeError::InvalidResponseBody => "response body invalid.".to_string(),
        };
        write!(f, "{wr}")
    }
}
