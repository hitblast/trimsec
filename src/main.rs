use std::{env, process};

use trimsec::Config;

const BOLD: &str = "\u{001b}[1m";
const GREEN: &str = "\u{001b}[32m";
const RED: &str = "\u{001b}[31m";
const RESET: &str = "\u{001b}[0m";

// Helper functions for printing.
fn print_error<T: std::fmt::Display>(msg: T) {
    eprintln!("{}[ERROR]{} {}", RED, RESET, msg);
}

// Main runner entrypoint.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ts <duration> <multiplier>");
        println!("       ts 1h2m 1.5x");
        println!("       ts 2d1h3m 1.25");
        return;
    }

    let duration = &args[1];
    let multiplier = &args[2];

    let config = Config::new(duration, multiplier).unwrap_or_else(|err| {
        print_error(err);
        process::exit(1);
    });

    let result = trimsec::trim(config);
    match result {
        Ok((new_duration, time_saved, splits)) => {
            // Display new duration.
            if time_saved > 0.0 {
                let parsed = trimsec::parse_time(new_duration);

                let message = format!(
                    "\nWould finish in: {} ",
                    if splits > 1 {
                        format!("{parsed} (all {splits} durations)")
                    } else {
                        parsed
                    }
                );
                println!("{}", message);
            } else {
                println!("No time saved. Would finish in linear time.");
                return;
            }

            // Display remaining time in day.
            let remaining = trimsec::calculate_remaining(new_duration);
            println!(
                "Day left afterwards: {} ",
                if remaining == 0.0 {
                    "0s".to_string()
                } else {
                    trimsec::parse_time(remaining)
                }
            );

            // Display saved time.
            if time_saved > 0.0 {
                let parsed = trimsec::parse_time(time_saved);
                println!("{GREEN}{BOLD}Saved {parsed}!{RESET}\n");
            }
        }
        Err(e) => {
            print_error(e);
            process::exit(1);
        }
    }
}
