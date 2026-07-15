use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum TConfigError {
    PathReadFailure(String),
    NonexistentPath(String),
    ParseFailed(PathBuf),
}

impl Display for TConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TConfigError::PathReadFailure(e) => write!(f, "failed to read path to string: {e}"),
            TConfigError::NonexistentPath(e) => write!(f, "failed to fetch config path: {e}"),
            TConfigError::ParseFailed(p) => write!(f, "could not parse file at path: {p:?}"),
        }
    }
}

#[derive(Debug)]
pub enum TTimeError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InvalidMultiplierFormat,
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
        }
    }
}

#[derive(Debug)]
pub enum TYoutubeError {
    Reqwest,
    ResponseBodyParseFailure,
    ItemNotFound,
    InvalidPlaylist(String),
    InvalidMaxSize((usize, usize)),
}

impl Display for TYoutubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TYoutubeError::Reqwest => {
                write!(
                    f,
                    "error performing request (check YouTube API key and internet connection)."
                )
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
