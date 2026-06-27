use clap::Parser;
use trimsec::cli::Args;

fn main() {
    let args = Args::parse();

    // command invocation
    let result = args.command.run();

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
