use clap::Parser;
use trimsec::{
    cli::{Args, args::Command},
    commands::Runnable,
};

fn main() {
    let args = Args::parse();

    // command invocation
    let result = match &args.command {
        Command::Trim(cmd) => cmd.run(),
        Command::Yt(cmd) => cmd.run(),
    };

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
