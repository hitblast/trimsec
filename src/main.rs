// imports
use colored::*;
use std::env;
use std::process;

use trimsec::Config;

// run the cli
fn main() {
    // parse the arguments
    let args: Vec<String> = env::args().collect();

    // create a new Config struct
    let config = Config::new(&args).unwrap_or_else(|err| {
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
