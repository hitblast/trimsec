use std::error::Error;
use std::process;

// The primary run function.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result = trim(config);

    println!("Reduced time: {}", result.0);
    println!("Saved {}!", result.1);

    Ok(())
}

/*

The Config struct is used to store the duration and multiplier values that are parsed from the command line arguments.
The Config::new function is used to parse the command line arguments and return a Config struct.
The parse_duration function is used to parse the duration string and return the total number of seconds.

*/

pub struct Config {
    pub duration: u64,
    pub multiplier: f64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // close immediately if sufficient arguments aren't passed
        if args.len() < 3 {
            return Err("git gud, you need to provide a duration and a multiplier");
        }

        // format conversion
        // firstly, parse the duration to *only* seconds and then convert it to a string
        let duration = args[1].clone();
        let duration_in_seconds = parse_duration(&duration).unwrap_or_else(|err| {
            eprintln!("Problem parsing duration: {}", err);
            process::exit(1);
        });

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

// Function to pass the duration string and return the total seconds.
fn parse_duration(duration: &str) -> Result<u64, &str> {
    let mut total_seconds = 0u64;
    let mut current_number = String::new();

    for c in duration.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else {
            let number: u64 = current_number
                .parse()
                .map_err(|_| "git gud, duration must be an integer")?;
            current_number.clear();
            total_seconds += match c {
                's' => number,
                'm' => number * 60,
                'h' => number * 3600,
                'd' => number * 86400,
                _ => return Err(
                    "You must specify duration in seconds (s), minutes (m), days (d) or hours (h).",
                ),
            };
        }
    }

    if !current_number.is_empty() {
        return Err("Invalid duration format");
    }

    Ok(total_seconds)
}

// Function to parse the time format in string.
pub fn parse_time(time: f64) -> String {
    let mut time_string = String::new();

    let hours = time as u64 / 3600;
    let minutes = time as u64 % 3600 / 60;
    let seconds = time as u64 % 60;

    for (i, time) in [hours, minutes, seconds].iter().enumerate() {
        if *time != 0 {
            time_string.push_str(&format!(
                "{}{}",
                time,
                match i {
                    0 => "h",
                    1 => "m",
                    2 => "s",
                    _ => "",
                }
            ));
        }
    }

    time_string
}

// trim() function to trim time!
pub fn trim<'a>(config: Config) -> (String, String) {
    let duration = config.duration as f64;
    let multiplier = config.multiplier;

    let result = duration / multiplier;
    let result_string = parse_time(result);

    let saved = duration - result;
    let saved_string = parse_time(saved);

    (result_string, saved_string)
}
