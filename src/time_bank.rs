// Imports.
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

// File for handling data storage for the time bank.
const BANK_FILE: &str = ".time_bank.json";

/// A typical time bank entry struct.
#[derive(Serialize, Deserialize, Debug)]
pub struct BankEntry {
    /// Store the date as a string (YYYY-MM-DD)
    pub date: String,
    /// Saved time in seconds for that day.
    pub saved_time: f64,
}

/// Represents a time bank.
/// This struct should only be used by the front CLI of trimsec.
#[derive(Serialize, Deserialize, Debug)]
pub struct TimeBank {
    pub entries: Vec<BankEntry>,
}

// Time bank implementation.
impl TimeBank {
    /// Load the time bank from the file. If the file does not exist, return an empty bank.
    pub fn load() -> Result<TimeBank, Box<dyn std::error::Error>> {
        match fs::read_to_string(BANK_FILE) {
            Ok(content) => {
                let bank: TimeBank = serde_json::from_str(&content)?;
                Ok(bank)
            }
            Err(ref e) if e.kind() == ErrorKind::NotFound => Ok(TimeBank { entries: vec![] }),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Add saved time (in seconds) from the current session into today’s entry.
    pub fn add_time(&mut self, saved_seconds: f64) {
        let today = Local::now().format("%Y-%m-%d").to_string();

        // Find the entry for today if it exists.
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.date == today) {
            entry.saved_time += saved_seconds;
        } else {
            // Otherwise, create a new entry for today.
            self.entries.push(BankEntry {
                date: today,
                saved_time: saved_seconds,
            });
        }
    }

    /// Calculate the total saved time over all entries.
    pub fn total_saved(&self) -> f64 {
        self.entries.iter().map(|entry| entry.saved_time).sum()
    }

    /// Return the absolute path to the bank file.
    pub fn bank_file_path(&self) -> PathBuf {
        PathBuf::from(BANK_FILE)
    }

    /// Save the current bank state to the designated JSON file.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(BANK_FILE, json)?;
        Ok(())
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_time() {
        let mut bank = TimeBank { entries: vec![] };
        bank.add_time(100.0);
        // Add more saved seconds – should accumulate on the same day.
        bank.add_time(50.0);
        assert_eq!(bank.entries.len(), 1);
        assert_eq!(bank.entries[0].saved_time, 150.0);
    }
}
