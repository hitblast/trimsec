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
            if !args.time_saved_only {
                let duration_message = if splits > 1 {
                    format!(
                        "\nNew duration: {} ({} splits)",
                        new_duration.yellow(),
                        splits
                    )
                } else {
                    format!("\nNew duration: {}", new_duration.yellow())
                };
                println!("{}", duration_message);
            }

            // saved time
            if !args.duration_only {
                println!("Saved {}!\n", time_saved.green());
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "ERROR".red(), e);
            process::exit(1);
        }
    }
}
