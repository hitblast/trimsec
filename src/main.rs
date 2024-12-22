// Imports.
use clap::Parser;
use colored::*;
use std::process;

use trimsec::Config;

// Struct for parsing.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The duration of the content to be recalculated.
    duration: String,
    /// The multiplier to apply.
    multiplier: String,
}

// Runner.
fn main() {
    let args = Cli::parse();

    // The Config struct is created here and the rest is handled by lib.rs.
    let config = Config::new(&args.duration, &args.multiplier).unwrap_or_else(|err| {
        eprintln!("{}: {}", "ERROR".red(), err);
        process::exit(1);
    });

    let result = trimsec::run(config);
    match result {
        Ok((dur, saved)) => {
            println!("\nReduced time: {}", dur.yellow());
            println!("Saved {}!\n", saved.green());
        }
        Err(e) => {
            eprintln!("{}: {}", "ERROR".red(), e);
            process::exit(1);
        }
    }
}
