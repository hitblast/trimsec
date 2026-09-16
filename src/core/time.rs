use chrono::{Datelike, TimeZone};
use std::{fmt::Display, iter::Sum, ops::Sub};

use crate::errors::TTimeError;

fn parse_multiplier(multiplier_user: &str) -> Result<f64, TTimeError> {
    let multiplier = if let Some(stripped) = multiplier_user.strip_suffix('x') {
        stripped
    } else {
        multiplier_user
    };

    let multiplier_value: f64 = multiplier
        .parse()
        .map_err(|_| TTimeError::InvalidMultiplierFormat)?;

    if !(1.0..100.0).contains(&multiplier_value) {
        Err(TTimeError::MultiplierOutOfRange)
    } else {
        Ok(multiplier_value)
    }
}

#[derive(PartialEq, Debug, PartialOrd, Default)]
pub struct TDuration {
    seconds: f64,
    saved_time: f64,
    splits: u64,
}

impl TDuration {
    pub fn seconds(&self) -> f64 {
        self.seconds
    }
    pub fn splits(&self) -> u64 {
        self.splits
    }
    pub fn saved_time(&self) -> f64 {
        self.saved_time
    }

    pub fn trim(&mut self, multiplier: &str) -> Result<(), TTimeError> {
        let multiplier_value = parse_multiplier(multiplier)?;

        let old = self.seconds();
        self.seconds = old / multiplier_value;
        self.saved_time += old - self.seconds();

        Ok(())
    }

    pub fn parse_str(duration_str: &str) -> Result<Self, TTimeError> {
        let mut seconds = 0f64;
        let mut splits = 0;

        for part in duration_str.split('+') {
            let mut current_number = String::new();
            let mut part_seconds = 0f64;

            for c in part.chars() {
                if c.is_ascii_digit() || c == '.' {
                    current_number.push(c);
                } else if c.is_whitespace() {
                    continue;
                } else {
                    let number: f64 = current_number
                        .parse()
                        .map_err(|_| TTimeError::NegativeDuration)?;
                    current_number.clear();
                    part_seconds += match c {
                        's' => number,
                        'm' => number * 60.0,
                        'h' => number * 3600.0,
                        'd' => number * 86400.0,
                        _ => return Err(TTimeError::InvalidTimeUnit),
                    };
                }
            }

            if !current_number.is_empty() {
                return Err(TTimeError::InvalidDurationFormat);
            }

            seconds += part_seconds;
            splits += 1;
        }

        Ok(Self {
            seconds,
            splits,
            saved_time: 0.0,
        })
    }
}

impl Display for TDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.seconds().to_string_duration())
    }
}

pub trait ToStringTime {
    fn to_string_duration(self) -> String;
}

impl ToStringTime for f64 {
    fn to_string_duration(self) -> String {
        let mut time_string = String::new();

        let days = self as u64 / 86400;
        let hours = (self as u64 % 86400) / 3600;
        let minutes = (self as u64 % 3600) / 60;
        let seconds = self as u64 % 60;

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
}

impl Sub for TDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            seconds: self.seconds - rhs.seconds(),
            saved_time: self.saved_time,
            splits: self.splits,
        }
    }
}

impl Sub<&TDuration> for &TDuration {
    type Output = TDuration;

    fn sub(self, rhs: &TDuration) -> Self::Output {
        TDuration {
            seconds: self.seconds - rhs.seconds(),
            saved_time: self.saved_time,
            splits: self.splits,
        }
    }
}

impl From<(f64, u64)> for TDuration {
    fn from(value: (f64, u64)) -> Self {
        TDuration {
            seconds: value.0,
            saved_time: 0.0,
            splits: value.1,
        }
    }
}

impl Sum for TDuration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            TDuration {
                seconds: 0.0,
                splits: 0,
                saved_time: 0.0,
            },
            |x, y| TDuration {
                seconds: x.seconds + y.seconds,
                splits: x.splits + y.splits,
                saved_time: x.saved_time + y.saved_time,
            },
        )
    }
}

impl Eq for TDuration {}

impl Ord for TDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seconds.total_cmp(&other.seconds)
    }
}

#[must_use]
pub fn time_in_day_after(duration: f64) -> f64 {
    let now = chrono::Local::now();
    let end_of_day = chrono::Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .unwrap();
    let time_passed = end_of_day.signed_duration_since(now).num_seconds() as f64;

    if time_passed > duration {
        time_passed - duration
    } else {
        0.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(TDuration::parse_str("1s").unwrap(), (1.0, 1).into());
        assert_eq!(TDuration::parse_str("1m").unwrap(), (60.0, 1).into());
        assert_eq!(TDuration::parse_str("1h").unwrap(), (3600.0, 1).into());
        assert_eq!(TDuration::parse_str("1d").unwrap(), (86400.0, 1).into());
        assert_eq!(
            TDuration::parse_str("1d1h1m1s").unwrap(),
            (90061.0, 1).into()
        );
        assert_eq!(
            TDuration::parse_str("1h+1m+1s").unwrap(),
            (3661.0, 3).into()
        );
        assert_eq!(TDuration::parse_str("1.5h").unwrap(), (5400.0, 1).into());
        assert_eq!(
            TDuration::parse_str("1.5h+30m").unwrap(),
            (7200.0, 2).into()
        );
        assert!(TDuration::parse_str("1x").is_err());
        assert!(TDuration::parse_str("1").is_err());
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(1.0.to_string_duration(), "1s");
        assert_eq!(60.0.to_string_duration(), "1m");
        assert_eq!(3600.0.to_string_duration(), "1h");
        assert_eq!(86400.0.to_string_duration(), "1d");
        assert_eq!(90061.0.to_string_duration(), "1d1h1m1s");
    }

    #[test]
    fn test_parse_multiplier() {
        assert_eq!(parse_multiplier("1x").unwrap(), 1.0);
        assert_eq!(parse_multiplier("2.5x").unwrap(), 2.5);
        assert_eq!(parse_multiplier("2.5").unwrap(), 2.5);
        assert!(parse_multiplier("2.5d").is_err());
    }

    #[test]
    fn test_trim() {
        let mut duration = TDuration::parse_str("1d").unwrap();
        duration.trim("2x").unwrap();

        assert_eq!(
            duration,
            TDuration {
                seconds: 43200.0,
                saved_time: 43200.0,
                splits: 1
            }
        );
    }
}
