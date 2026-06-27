use std::fmt::Display;

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
                "Specify duration in seconds (s), minutes (m), hours (h), or days (d)"
            ),
            Self::InvalidDurationFormat => write!(f, "Invalid duration format!"),
            Self::NegativeDuration => write!(f, "Duration must be a positive value."),
            Self::InvalidMultiplierFormat => {
                write!(f, "Multiplier must be a positive float.")
            }
            Self::MultiplierOutOfRange => {
                write!(f, "Multiplier must be greater than 1x and less than 100x.")
            }
        }
    }
}

#[derive(Debug)]
pub enum TYoutubeError {
    Reqwest(reqwest::Error),
    ItemNotFound,
    InvalidPlaylist(String),
    InvalidMaxSize((usize, usize)),
}

impl Display for TYoutubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TYoutubeError::Reqwest(error) => {
                write!(f, "Error invoking reqwest functions: {error}")
            }
            TYoutubeError::ItemNotFound => {
                write!(f, "Given YouTube video item was not found in API response.")
            }
            TYoutubeError::InvalidPlaylist(id) => {
                write!(f, "Invalid playlist: {id}")
            }
            TYoutubeError::InvalidMaxSize((given, max)) => {
                write!(
                    f,
                    "Max items ({given}) is larger than the length of the playlist ({max})."
                )
            }
        }
    }
}
