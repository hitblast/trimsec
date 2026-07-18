use clap::Parser;
use trimsec::cli::Args;

fn main() {
    let args = Args::parse();

    if let Err(err) = args.command.run(args.clip, args.color) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
