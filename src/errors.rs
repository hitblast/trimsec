use std::fmt::Display;

/// The error enum for generating error messages later on.
#[derive(Debug)]
pub enum TrimsecError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InsufficientArgumentsProvided,
    InvalidMultiplierFormat,
    MultiplierOutOfRange,
    TimeBankUnloaded,
}

impl Display for TrimsecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTimeUnit => write!(
                f,
                "Specify duration in seconds (s), minutes (m), hours (h), or days (d)"
            ),
            Self::InvalidDurationFormat => write!(f, "Invalid duration format!"),
            Self::InsufficientArgumentsProvided => write!(
                f,
                "You need to provide a duration and a multiplier (e.g. `trimsec 1h 2x`)."
            ),
            Self::NegativeDuration => write!(f, "Duration must be a positive value."),
            Self::InvalidMultiplierFormat => {
                write!(f, "Multiplier must be a positive float.")
            }
            Self::MultiplierOutOfRange => write!(
                f,
                "Multiplier must be greater than 1x and less than 100x."
            ),
            Self::TimeBankUnloaded => write!(f, "Time bank was not loaded."),
        }
    }
}
