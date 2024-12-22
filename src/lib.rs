use std::process;
use std::fmt::Display;

#[derive(Debug)]
pub enum TrimsecError {
    InvalidDurationFormat,
    /// Called when omething other than `s`, `m`, `h`, `d` was used
    InvalidTimeUnit,
    NegativeDuration,
    InsufficientArgumentsProvided,
}

impl Display for TrimsecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTimeUnit => write!(f, "{}", "Specify duration in seconds (s), minutes (m), hours (h), or days (d)"),
            Self::InvalidDurationFormat => write!(f, "{}", "Invalid duration format!"),
            Self::InsufficientArgumentsProvided => write!(f, "{}", "You need to provide a duration and a multiplier (e.g. `trimsec 1h 2x`)."),
            Self::NegativeDuration => write!(f, "{}", "Duration must be a positive value.")
        }
    }
}

// The primary run function.
pub fn run(config: Config) -> Result<(String, String), TrimsecError> {
    Ok(trim(config))
}

/// The configuration struct used for
/// parsing and storing the duration and multiplier.
pub struct Config {
    pub duration: u64,
    pub multiplier: f64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, TrimsecError> {
        // close immediately if sufficient arguments aren't passed
        if args.len() < 3 {
            return Err(TrimsecError::InsufficientArgumentsProvided);
        }

        // format conversion
        // firstly, parse the duration to *only* seconds and then convert it to a string
        let duration = args[1].clone();
        let duration_in_seconds = parse_duration(&duration)?;

        // remove any multiplier formats and convert to a float
        let multiplier_unformatted = args[2].clone();
        let multiplier = if multiplier_unformatted.ends_with('x') {
            &multiplier_unformatted[..multiplier_unformatted.len() - 1]
        } else {
            &multiplier_unformatted
        };

        let multiplier_value: f64 = multiplier.parse().unwrap_or_else(|_| {
            eprintln!("Multiplier must be a positive float");
            process::exit(1);
        });

        match multiplier_value {
            ..0.0 => {
                eprintln!("Multiplier must be a positive float.");
                process::exit(1);
            }
            0.0..=1.0 => {
                eprintln!("Multiplier must be greater than 1x.");
                process::exit(1);
            }
            100.0..=f64::INFINITY => {
                eprintln!("Multiplier must be less than 100x.");
                process::exit(1);
            }
            _ => (),
        }

        // return the Config struct
        Ok(Config {
            duration: duration_in_seconds,
            multiplier: multiplier_value,
        })
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

/// Calculate how much time has been saved by using a multiplier.
pub fn trim(config: Config) -> (String, String) {
    let duration = config.duration as f64;
    let multiplier = config.multiplier;

    let result = duration / multiplier;
    let result_string = parse_time(result);

    let saved = duration - result;
    let saved_string = parse_time(saved);

    (result_string, saved_string)
}

// Function to pass the duration string and return the total seconds.
fn parse_duration(duration: &str) -> Result<u64, TrimsecError> {
    let mut total_seconds = 0u64;
    let mut current_number = String::new();

    for c in duration.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if c.is_whitespace() {
            continue;
        } else {
            let number: u64 = current_number
                .parse()
                .map_err(|_| TrimsecError::NegativeDuration)?;
            current_number.clear();
            total_seconds += match c {
                's' => number,
                'm' => number * 60,
                'h' => number * 3600,
                'd' => number * 86400,
                _ => {
                    return Err(TrimsecError::InvalidTimeUnit)
                }
            };
        }
    }

    if !current_number.is_empty() {
        return Err(TrimsecError::InvalidDurationFormat);
    }

    Ok(total_seconds)
}
