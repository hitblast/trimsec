use clap::Parser;
use trimsec::cli::Args;

fn main() {
    let args = Args::parse();

    let result = args.command.run(args.no_clip);

    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
