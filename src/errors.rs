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
pub enum TTimeError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InvalidMultiplierFormat,
    EmptyDurationString,
    MultiplierOutOfRange,
}

impl Display for TTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTimeUnit => write!(
                f,
                "specify duration in seconds (s), minutes (m), hours (h), or days (d)"
            ),
            Self::InvalidDurationFormat => write!(f, "invalid duration format!"),
            Self::NegativeDuration => write!(f, "duration must be a positive value."),
            Self::InvalidMultiplierFormat => {
                write!(f, "multiplier must be a positive float.")
            }
            Self::MultiplierOutOfRange => {
                write!(f, "multiplier must be greater than 1x and less than 100x.")
            }
            Self::EmptyDurationString => {
                write!(f, "empty duration-string passed.")
            }
        }
    }
}

#[derive(Debug)]
pub enum TYoutubeError {
    UreqError(ureq::Error),
    RequestUriParseError(String),
    ResponseBodyParseFailure,
    ItemNotFound,
    InvalidPlaylist(String),
    InvalidMaxSize((usize, usize)),
}

impl Display for TYoutubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TYoutubeError::UreqError(e) => {
                write!(f, "error performing request: {e}.")
            }
            TYoutubeError::RequestUriParseError(e) => {
                write!(f, "error parsing request Uri: {e}")
            }
            TYoutubeError::ItemNotFound => {
                write!(f, "given YouTube video item was not found in API response.")
            }
            TYoutubeError::InvalidPlaylist(id) => {
                write!(f, "invalid playlist: {id}")
            }
            TYoutubeError::InvalidMaxSize((given, max)) => {
                write!(
                    f,
                    "max items ({given}) is larger than the length of the playlist ({max})."
                )
            }
            TYoutubeError::ResponseBodyParseFailure => write!(f, "failed to parse response body."),
        }
    }
}
