use clap::Parser;
use colored::*;
use std::{fs, process};

use trimsec::Config;

mod time_bank;
use time_bank::TimeBank;

mod args;
use args::{BankCommands, Cli, Commands};

// Helper functions for printing.
fn print_error<T: std::fmt::Display>(msg: T) {
    eprintln!("{} {}", "[ERROR]".red(), msg);
}

fn print_warning<T: std::fmt::Display>(msg: T) {
    eprintln!("{} {}", "[WARNING]".yellow(), msg);
}

// Main runner entrypoint.
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
            let config = Config::new(&duration, &multiplier).unwrap_or_else(|err| {
                print_error(err);
                process::exit(1);
            });

            let result = trimsec::trim(config);
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
                            format!(" Saved {}{}!\n", if emoji { "⏰ " } else { "" }, parsed)
                                .green()
                        );

                        // Load the time bank, update it, and save.
                        match TimeBank::load() {
                            Ok(mut bank) => {
                                bank.add_time(time_saved);
                                if let Err(e) = bank.save() {
                                    print_warning(format!("Could not update time bank: {}", e));
                                } else {
                                    println!(
                                        "{}",
                                        format!(
                                            " [Total saved time: {}]",
                                            trimsec::parse_time(bank.total_saved())
                                        )
                                        .dimmed()
                                    );
                                }
                            }
                            Err(e) => {
                                print_warning(format!("Could not load time bank: {}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    print_error(e);
                    process::exit(1);
                }
            }
        }

        Commands::Bank { bank_command } => match bank_command {
            Some(BankCommands::Show) | None => match TimeBank::load() {
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
                    print_error(format!("Could not load time bank: {}", e));
                }
            },
            Some(BankCommands::Reset) => {
                let bank = TimeBank { entries: vec![] };
                if let Err(e) = bank.save() {
                    print_error(format!("Could not reset time bank: {}", e));
                    process::exit(1);
                } else {
                    println!("Time bank has been reset.");
                }
            }
            Some(BankCommands::Path) => match TimeBank::load() {
                Ok(_) => match fs::canonicalize(TimeBank::bank_file_path()) {
                    Ok(path) => println!("{}", path.display()),
                    Err(_) => {
                        print_error("Could not get canonical path. Time bank was not initialized by trimsec.");
                    }
                },
                Err(e) => {
                    print_error(format!("Could not load time bank: {}", e));
                }
            },
        },
    }
}
