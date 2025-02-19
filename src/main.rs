// Imports.
use clap::{Parser, Subcommand};
use colored::*;
use std::process;

use trimsec::Config;

mod time_bank;
use time_bank::TimeBank;

/// trimsec - Strategic (& fast) content consumption planner.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate saved time using a multiplier over a given duration.
    Trim {
        /// The duration of the content (e.g. "1h30m" or "1h+30m").
        duration: String,
        /// The speed multiplier (e.g. "1.5x").
        multiplier: String,
        /// Only show new duration.
        #[arg(short, long)]
        duration_only: bool,
        /// Only show saved time.
        #[arg(short, long)]
        time_saved_only: bool,
        /// Use seconds as the time unit.
        #[arg(short, long)]
        seconds: bool,
        /// Show emojis in the output.
        #[arg(short, long)]
        emoji: bool,
    },
    /// Manage or view your time bank data.
    Bank {
        #[command(subcommand)]
        bank_command: Option<BankCommands>,
    },
}

#[derive(Subcommand)]
enum BankCommands {
    /// Show the current time bank details.
    Show,
    /// Reset (clear) the time bank.
    Reset,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Trim {
            duration,
            multiplier,
            duration_only,
            time_saved_only,
            seconds,
            emoji,
        } => {
            // Create the Config – in case of errors exit.
            let config = Config::new(&duration, &multiplier).unwrap_or_else(|err| {
                eprintln!("{}: {}", "ERROR".red(), err);
                process::exit(1);
            });

            // Calculate using trimsec logic.
            let result = trimsec::run(config);
            match result {
                Ok((new_duration, time_saved, splits)) => {
                    // Display new duration.
                    if !time_saved_only && time_saved > 0.0 {
                        let parsed = if seconds {
                            format!("{:.2}s", new_duration)
                        } else {
                            trimsec::parse_time(new_duration)
                        };

                        let message = format!(
                            " New duration: {}{} ",
                            if emoji { "⏳ " } else { "" },
                            if splits > 1 {
                                format!("{} ({} splits)", parsed, splits)
                            } else {
                                parsed
                            }
                        );
                        println!("{}", message);
                    }

                    // Display remaining time in day unless limited by flags.
                    if !duration_only && !time_saved_only {
                        let remaining = trimsec::calculate_remaining(new_duration);
                        println!(
                            " Time leftover in day: {}{} ",
                            if emoji {
                                if remaining > 0.0 {
                                    "🟢 "
                                } else {
                                    "🔴 "
                                }
                            } else {
                                ""
                            },
                            if seconds {
                                format!("{:.2}s", remaining)
                            } else {
                                if remaining == 0.0 {
                                    "0s".to_string()
                                } else {
                                    trimsec::parse_time(remaining)
                                }
                            }
                        );
                    }

                    // Display saved time and update time bank.
                    if !duration_only && time_saved > 0.0 {
                        let parsed = if seconds {
                            format!("{:.2}s", time_saved)
                        } else {
                            trimsec::parse_time(time_saved)
                        };
                        println!(
                            "{}",
                            format!(" Saved {}{}! ", if emoji { "⏰ " } else { "" }, parsed)
                                .green()
                        );

                        // Load the time bank, update it, and save.
                        match TimeBank::load() {
                            Ok(mut bank) => {
                                bank.add_time(time_saved);
                                if let Err(e) = bank.save() {
                                    eprintln!(
                                        "{}: Could not update time bank: {}",
                                        "WARNING".yellow(),
                                        e
                                    );
                                } else {
                                    println!(
                                        "Time bank updated. Total saved time: {}",
                                        trimsec::parse_time(bank.total_saved())
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "{}: Could not load time bank: {}",
                                    "WARNING".yellow(),
                                    e
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "ERROR".red(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Bank { bank_command } => {
            match bank_command {
                Some(BankCommands::Show) | None => {
                    // By default, show time bank details.
                    match TimeBank::load() {
                        Ok(bank) => {
                            if bank.entries.is_empty() {
                                println!("Time bank is empty.");
                            } else {
                                println!("Time Bank Details:");
                                for entry in &bank.entries {
                                    println!(
                                        "  {}: {}",
                                        entry.date,
                                        trimsec::parse_time(entry.saved_time)
                                    );
                                }
                                println!(
                                    "\nTotal saved time: {}",
                                    trimsec::parse_time(bank.total_saved())
                                );
                            }
                        }
                        Err(e) => {
                            eprintln!("{}: Could not load time bank: {}", "ERROR".red(), e);
                        }
                    }
                }
                Some(BankCommands::Reset) => {
                    // Reset the bank: overwrite with an empty bank structure.
                    let bank = TimeBank { entries: vec![] };
                    if let Err(e) = bank.save() {
                        eprintln!("{}: Could not reset time bank: {}", "ERROR".red(), e);
                        process::exit(1);
                    } else {
                        println!("Time bank has been reset.");
                    }
                }
            }
        }
    }
}
