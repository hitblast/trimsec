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
pub enum TParseDurationError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InvalidMultiplierFormat,
    EmptyDurationString,
    MultiplierOutOfRange,
}

impl Display for TParseDurationError {
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
pub enum TFetchClipboardError {
    ClipboardMutexPoisoned(String),
    ClipboardContentFetchFailed(String),
    Clipboard(String),
}

impl Display for TFetchClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            TFetchClipboardError::Clipboard(e) => {
                &format!("clipboard could not be initialized: {e}")
            }
            TFetchClipboardError::ClipboardMutexPoisoned(e) => {
                &format!("clipboard mutex lock poisoned: {e}")
            }
            TFetchClipboardError::ClipboardContentFetchFailed(e) => {
                &format!("failed to fetch clipboard content: {e}")
            }
        };
        write!(f, "{}", wr)
    }
}

#[derive(Debug)]
pub enum TYtClientError {
    // Youtube Data API
    UreqError(String),
    RequestUriParseError(String),
    InvalidResponseBody(String),
    ItemNotFound,
    InvalidPlaylist(String),
    InvalidMaxSize((usize, usize)),
    // yt-dlp
    UnsupportedError(&'static str),
    StderrReadError(String),
    StdoutTakeError,
    StderrTakeError,
    CmdSpawnError(String),
    BufReaderError(String),
    ErroneousCmdOutput(String),
    DeserializationError(String),
}

impl Display for TYtClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            TYtClientError::UreqError(e) => format!("error performing request: {e}."),
            TYtClientError::RequestUriParseError(e) => format!("error parsing request Uri: {e}"),
            TYtClientError::ItemNotFound => {
                "given YouTube video item was not found in API response.".to_string()
            }
            TYtClientError::InvalidPlaylist(id) => format!("invalid playlist: {id}"),
            TYtClientError::InvalidMaxSize((given, max)) => {
                format!("max items ({given}) is larger than the length of the playlist ({max}).")
            }
            TYtClientError::InvalidResponseBody(e) => format!("response body invalid: {e}"),
            TYtClientError::UnsupportedError(e) => format!("unsupported behavior: {e}"),
            TYtClientError::ErroneousCmdOutput(e) => {
                format!("erroneous output while executing command: {e}")
            }
            TYtClientError::StderrReadError(e) => format!("error reading stderr: {e}"),
            TYtClientError::StderrTakeError => "error taking stderr".to_string(),
            TYtClientError::StdoutTakeError => "error taking stdout".to_string(),
            TYtClientError::BufReaderError(e) => format!("error reading stdout: {e}"),
            TYtClientError::CmdSpawnError(e) => format!("error spawning command: {e}"),
            TYtClientError::DeserializationError(e) => format!("error deserializing response: {e}"),
        };
        write!(f, "{wr}")
    }
}

#[derive(Debug)]
pub enum TYtClientInitError {
    Ytdlp(String),
    YtDataApi(String),
}

impl Display for TYtClientInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wr = match self {
            TYtClientInitError::Ytdlp(e) => format!("error initializing yt-dlp client: {e}"),
            TYtClientInitError::YtDataApi(e) => {
                format!("error initializing youtube data api client: {e}")
            }
        };
        write!(f, "{wr}")
    }
}
