use std::fmt::Display;

use chrono::{Datelike, TimeZone};

/// The primary run function.
pub fn run(config: Config) -> Result<(f64, f64, i64), TrimsecError> {
    Ok(trim(config))
}

/// Calculate how much time has been saved by using a multiplier.
/// Returns a tuple with the new duration, saved time, and the number of splits.
pub fn trim(config: Config) -> (f64, f64, i64) {
    let old_duration = config.duration;
    let multiplier = config.multiplier;

    let new_duration = old_duration / multiplier;
    let saved_time = old_duration - new_duration;

    (new_duration, saved_time, config.splits)
}

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
                "{}",
                "Specify duration in seconds (s), minutes (m), hours (h), or days (d)"
            ),
            Self::InvalidDurationFormat => write!(f, "{}", "Invalid duration format!"),
            Self::InsufficientArgumentsProvided => write!(
                f,
                "{}",
                "You need to provide a duration and a multiplier (e.g. `trimsec 1h 2x`)."
            ),
            Self::NegativeDuration => write!(f, "{}", "Duration must be a positive value."),
            Self::InvalidMultiplierFormat => {
                write!(f, "{}", "Multiplier must be a positive float.")
            }
            Self::MultiplierOutOfRange => write!(
                f,
                "{}",
                "Multiplier must be greater than 1x and less than 100x."
            ),
            Self::TimeBankUnloaded => write!(f, "{}", "Time bank was not loaded."),
        }
    }
}

/// The configuration struct used for
/// parsing and storing the duration and multiplier.
pub struct Config {
    pub duration: f64,
    pub multiplier: f64,
    pub splits: i64,
}

impl Config {
    pub fn new(duration: &str, multiplier_user: &str) -> Result<Config, TrimsecError> {
        // format conversion
        let duration_tuple = parse_duration(&duration)?;
        let multiplier_value = parse_multiplier(multiplier_user)?;

        // return the Config struct
        Ok(Config {
            duration: duration_tuple.0,
            multiplier: multiplier_value,
            splits: duration_tuple.1,
        })
    }
}

/// Parse the multiplier string and return the multiplier value as a float.
fn parse_multiplier(multiplier_user: &str) -> Result<f64, TrimsecError> {
    let multiplier = if multiplier_user.ends_with('x') {
        &multiplier_user[..multiplier_user.len() - 1]
    } else {
        &multiplier_user
    };

    let multiplier_value: f64 = multiplier
        .parse()
        .map_err(|_| TrimsecError::InvalidMultiplierFormat)?;

    if multiplier_value < 1.0 || multiplier_value >= 100.0 {
        Err(TrimsecError::MultiplierOutOfRange)
    } else {
        Ok(multiplier_value)
    }
}

/// Convert seconds to a human-readable `String`.
pub fn parse_time(time: f64) -> String {
    let mut time_string = String::new();

    let days = time as u64 / 86400;
    let hours = (time as u64 % 86400) / 3600;
    let minutes = (time as u64 % 3600) / 60;
    let seconds = time as u64 % 60;

    for (i, time) in [days, hours, minutes, seconds].iter().enumerate() {
        if *time != 0 {
            time_string.push_str(&format!(
                "{}{}",
                time,
                match i {
                    0 => "d",
                    1 => "h",
                    2 => "m",
                    3 => "s",
                    _ => "",
                }
            ));
        }
    }

    time_string
}

/// Function to pass the duration string and return the total seconds.
fn parse_duration(duration: &str) -> Result<(f64, i64), TrimsecError> {
    let mut total_seconds = 0f64;
    let mut splits = 0;

    for part in duration.split('+') {
        let mut current_number = String::new();
        let mut part_seconds = 0f64;

        for c in part.chars() {
            if c.is_digit(10) || c == '.' {
                current_number.push(c);
            } else if c.is_whitespace() {
                continue;
            } else {
                let number: f64 = current_number
                    .parse()
                    .map_err(|_| TrimsecError::NegativeDuration)?;
                current_number.clear();
                part_seconds += match c {
                    's' => number,
                    'm' => number * 60.0,
                    'h' => number * 3600.0,
                    'd' => number * 86400.0,
                    _ => return Err(TrimsecError::InvalidTimeUnit),
                };
            }
        }

        if !current_number.is_empty() {
            return Err(TrimsecError::InvalidDurationFormat);
        }

        total_seconds += part_seconds;
        splits += 1;
    }

    Ok((total_seconds, splits))
}

/// Function to check the time efficiency for the current day.
pub fn calculate_remaining(trimmed_dur: f64) -> f64 {
    let now = chrono::Local::now();
    let end_of_day = chrono::Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .unwrap();
    let duration = end_of_day.signed_duration_since(now).num_seconds() as f64;

    if duration > trimmed_dur {
        duration - trimmed_dur
    } else {
        0.0
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s").unwrap(), (1.0, 1));
        assert_eq!(parse_duration("1m").unwrap(), (60.0, 1));
        assert_eq!(parse_duration("1h").unwrap(), (3600.0, 1));
        assert_eq!(parse_duration("1d").unwrap(), (86400.0, 1));
        assert_eq!(parse_duration("1d1h1m1s").unwrap(), (90061.0, 1));
        assert_eq!(parse_duration("1h+1m+1s").unwrap(), (3661.0, 3));
        assert_eq!(parse_duration("1.5h").unwrap(), (5400.0, 1));
        assert_eq!(parse_duration("1.5h+30m").unwrap(), (7200.0, 2));
        assert!(parse_duration("1x").is_err());
        assert!(parse_duration("1").is_err());
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time(1.0), "1s");
        assert_eq!(parse_time(60.0), "1m");
        assert_eq!(parse_time(3600.0), "1h");
        assert_eq!(parse_time(86400.0), "1d");
        assert_eq!(parse_time(90061.0), "1d1h1m1s");
    }

    #[test]
    fn test_config_new() {
        assert!(Config::new("1s", "2x").is_ok());
        assert!(Config::new("1m", "1.5x").is_ok());
        assert!(Config::new("1h", "1.25x").is_ok());
        assert!(Config::new("1d", "2x").is_ok());
        assert!(Config::new("1d1h1m1s", "2x").is_ok());
        assert!(Config::new("1.5h", "2x").is_ok());
        assert!(Config::new("1.5h+30m", "2x").is_ok());
        assert!(Config::new("1x", "2x").is_err());
        assert!(Config::new("1", "2x").is_err());
    }

    #[test]
    fn test_trim() {
        let config = Config::new("1d", "2x").unwrap();
        assert_eq!(trim(config), (43200.0, 43200.0, 1));
    }

    #[test]
    fn test_run() {
        let config = Config::new("1d", "2x").unwrap();
        let result = run(config).unwrap();
        assert_eq!(parse_time(result.0), "12h");
        assert_eq!(parse_time(result.1), "12h");
        assert_eq!(result.2, 1);
    }
}
