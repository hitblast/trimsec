// imports
use std::env;
use std::process;

use trimsec::Config;

// run the cli
fn main() {
    // parse the arguments
    let args: Vec<String> = env::args().collect();

    // create a new Config struct
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    // run the trimsec function
    if let Err(e) = trimsec::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
