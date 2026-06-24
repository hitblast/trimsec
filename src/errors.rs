use std::fmt::Display;

#[derive(Debug)]
pub enum TrimsecTimeError {
    InvalidDurationFormat,
    InvalidTimeUnit,
    NegativeDuration,
    InvalidMultiplierFormat,
    MultiplierOutOfRange,
}

impl Display for TrimsecTimeError {
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
