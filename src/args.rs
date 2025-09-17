use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum BankCommands {
    /// Show the current time bank details.
    Show,
    /// Reset (clear) the time bank.
    Reset,
    /// Return the absolute path to the bank file.
    Path,
}
