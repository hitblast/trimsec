use trimsec::args::get_cur_cmd;

fn main() {
    let eexit = |e| {
        eprintln!("{e}");
        std::process::exit(1);
    };

    match get_cur_cmd() {
        Ok((args, cmd)) => {
            if let Err(e) = cmd.run(&args) {
                eexit(e);
            }
        }
        Err(e) => {
            eexit(e);
        }
    }
}
