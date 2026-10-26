use std::fmt::Display;

#[derive(Debug)]
pub enum TConfigError {
    InvalidParentPath,
    UnavailableConfigPath,
    HomeNotFound,
    ConfigReadFailure(String),
    DeserializingFailed(String),
    SerializingFailed(String),
    SaveFailed(String),
}

impl Display for TConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            TConfigError::InvalidParentPath => "invalid parent path for file".to_string(),
            TConfigError::UnavailableConfigPath => "could not fetch absolute filepath".to_string(),
            TConfigError::HomeNotFound => "home  directory not found".to_string(),
            TConfigError::ConfigReadFailure(e) => format!("failed to read file: {e}"),
            TConfigError::DeserializingFailed(e) => {
                format!("parse failed: {e}")
            }
            TConfigError::SerializingFailed(e) => format!("could not serialize file: {e}"),
            TConfigError::SaveFailed(p) => format!("could not save file to path: {p:?}"),
        };
        write!(f, "{wr}")
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
pub enum FetchTClipStringError {
    ClipboardMutexPoisoned(String),
    ClipboardContentFetchFailed(String),
    Clipboard(String),
}

impl Display for FetchTClipStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            FetchTClipStringError::Clipboard(e) => {
                &format!("clipboard could not be initialized: {e}")
            }
            FetchTClipStringError::ClipboardMutexPoisoned(e) => {
                &format!("clipboard mutex lock poisoned: {e}")
            }
            FetchTClipStringError::ClipboardContentFetchFailed(e) => {
                &format!("failed to fetch clipboard content: {e}")
            }
        };
        write!(f, "{}", wr)
    }
}

#[derive(Debug)]
pub enum TYoutubeError {
    UreqError(String),
    RequestUriParseError(String),
    InvalidResponseBody(String),
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
            TYoutubeError::InvalidResponseBody(e) => format!("response body invalid: {e}"),
        };
        write!(f, "{wr}")
    }
}
