use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{AddAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::errors::ParseTDurationError;

pub fn parse_multiplier(multiplier: &str) -> Result<f64, ParseTDurationError> {
    let multiplier = if let Some(stripped) = multiplier.strip_suffix('x') {
        stripped
    } else {
        multiplier
    };

    let mult_value: f64 = multiplier
        .parse()
        .map_err(|_| ParseTDurationError::InvalidMultiplierFormat)?;

    if !(1.0..100.0).contains(&mult_value) {
        Err(ParseTDurationError::MultiplierOutOfRange)
    } else {
        Ok(mult_value)
    }
}

#[derive(Debug)]
pub struct TTime {
    left: u8,
    right: u8,
}

impl ToString for TTime {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct TDuration {
    seconds: f64,
    saved_time: f64,
    splits: u64,
}

impl Serialize for TDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        value.parse::<TDuration>().map_err(serde::de::Error::custom)
    }
}

impl Ord for TDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seconds.total_cmp(&other.seconds)
    }
}
impl PartialOrd for TDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TDuration {
    #[must_use]
    pub fn seconds(&self) -> f64 {
        self.seconds
    }
    #[must_use]
    pub fn splits(&self) -> u64 {
        self.splits
    }
    #[must_use]
    pub fn saved_time(&self) -> f64 {
        self.saved_time
    }
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.seconds() <= 0.0
    }

    pub fn trim(&mut self, multiplier: f64) {
        let old = self.seconds();
        self.seconds = old / multiplier;
        self.saved_time += old - self.seconds();
    }
}

impl FromStr for TDuration {
    type Err = ParseTDurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seconds = 0f64;
        let mut splits = 0;

        if s.is_empty() {
            return Err(ParseTDurationError::EmptyDurationString);
        }

        for part in s.split('+') {
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
                        .map_err(|_| ParseTDurationError::NegativeDuration)?;
                    current_number.clear();
                    part_seconds += match c {
                        's' => number,
                        'm' => number * 60.0,
                        'h' => number * 3600.0,
                        'd' => number * 86400.0,
                        _ => return Err(ParseTDurationError::InvalidTimeUnit),
                    };
                }
            }

            if !current_number.is_empty() {
                return Err(ParseTDurationError::InvalidDurationFormat);
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
            seconds: (self.seconds - rhs.seconds()).max(0.0),
            saved_time: self.saved_time,
            splits: self.splits,
        }
    }
}

impl Sub<&TDuration> for &TDuration {
    type Output = TDuration;

    fn sub(self, rhs: &TDuration) -> Self::Output {
        TDuration {
            seconds: (self.seconds - rhs.seconds()).max(0.0),
            saved_time: self.saved_time,
            splits: self.splits,
        }
    }
}

impl SubAssign<&TDuration> for TDuration {
    fn sub_assign(&mut self, rhs: &TDuration) {
        self.seconds = (self.seconds - rhs.seconds).max(0.0);
    }
}

impl From<f64> for TDuration {
    fn from(value: f64) -> Self {
        TDuration {
            seconds: value,
            saved_time: 0.0,
            splits: 1,
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

impl AddAssign<&TDuration> for TDuration {
    fn add_assign(&mut self, rhs: &TDuration) {
        self.seconds += rhs.seconds;
        self.splits += rhs.splits;
        self.saved_time += rhs.saved_time;
    }
}

impl Eq for TDuration {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    impl From<(f64, u64)> for TDuration {
        fn from(value: (f64, u64)) -> Self {
            TDuration {
                seconds: value.0,
                saved_time: 0.0,
                splits: value.1,
            }
        }
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!("1s".parse::<TDuration>().unwrap(), (1.0, 1).into());
        assert_eq!("1m".parse::<TDuration>().unwrap(), (60.0, 1).into());
        assert_eq!("1h".parse::<TDuration>().unwrap(), (3600.0, 1).into());
        assert_eq!("1d".parse::<TDuration>().unwrap(), (86400.0, 1).into());
        assert_eq!(
            "1d1h1m1s".parse::<TDuration>().unwrap(),
            (90061.0, 1).into()
        );
        assert_eq!("1h+1m+1s".parse::<TDuration>().unwrap(), (3661.0, 3).into());
        assert_eq!("1.5h".parse::<TDuration>().unwrap(), (5400.0, 1).into());
        assert_eq!("1.5h+30m".parse::<TDuration>().unwrap(), (7200.0, 2).into());
        assert!("1x".parse::<TDuration>().is_err());
        assert!("1".parse::<TDuration>().is_err());
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
        let mut duration = "1d".parse::<TDuration>().unwrap();
        duration.trim(2.0);

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
