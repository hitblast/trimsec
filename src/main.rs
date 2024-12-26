use clap::Parser;
use colored::*;
use std::process;

use trimsec::Config;

/// Struct for parsing.
#[derive(Parser)]
#[command(
    version,
    about,
    arg_required_else_help = true,
    long_about = None,
)]
struct Cli {
    /// The duration of the content to be recalculated.
    duration: String,
    /// The multiplier to apply.
    multiplier: String,
    /// Only show new duration.
    #[clap(short, long)]
    duration_only: bool,
    /// Only show saved time.
    #[clap(short, long)]
    time_saved_only: bool,
    /// Use seconds as the time unit.
    #[clap(short, long)]
    seconds: bool,
    /// Show emojis in the output.
    #[clap(short, long)]
    emoji: bool,
}

/// Runner.
fn main() {
    let args = Cli::parse();

    // The Config struct is created here and the rest is handled by lib.rs.
    let config = Config::new(&args.duration, &args.multiplier).unwrap_or_else(|err| {
        eprintln!("{}: {}", "ERROR".red(), err);
        process::exit(1);
    });

    let result = trimsec::run(config);
    match result {
        Ok((new_duration, time_saved, splits)) => {
            // duration
            if !args.time_saved_only && time_saved > 0.0 {
                let parsed = if args.seconds {
                    format!("{:.2}s", new_duration)
                } else {
                    trimsec::parse_time(new_duration)
                };

                let message = format!(
                    " New duration: {}{} ",
                    if args.emoji { "⏳ " } else { r#""# },
                    if splits > 1 {
                        format!("{} ({} splits)", parsed, splits)
                    } else {
                        parsed
                    }
                );

                println!("{}", message);
            }

            // remaining time in current day
            if !args.duration_only && !args.time_saved_only {
                let remaining = trimsec::calculate_remaining(new_duration);
                println!(
                    " Time leftover in day: {}{} ",
                    if args.emoji {
                        if remaining > 0.0 {
                            "🟢 "
                        } else {
                            "🔴 "
                        }
                    } else {
                        r#""#
                    },
                    if args.seconds {
                        format!("{:.2}s", remaining)
                    } else {
                        trimsec::parse_time(remaining)
                    }
                );
            }

            // saved time
            if !args.duration_only && time_saved > 0.0 {
                let parsed = if args.seconds {
                    format!("{:.2}s", time_saved)
                } else {
                    trimsec::parse_time(time_saved)
                };
                println!(
                    "{}",
                    format!(
                        " Saved {}{}! ",
                        if args.emoji { "⏰ " } else { r#""# },
                        parsed
                    )
                    .green()
                );
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "ERROR".red(), e);
            process::exit(1);
        }
    }
}
