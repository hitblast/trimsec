use trimsec::args::get_cur_cmd;

fn main() {
    let eexit = |e, is_error| {
        eprintln!("{}{e}", if is_error { "Error: " } else { "" });
        std::process::exit(1);
    };

    match get_cur_cmd() {
        Ok((args, cmd)) => {
            if let Err(e) = cmd.run(args) {
                eexit(e, true);
            }
        }
        Err(e) => {
            eexit(e, false);
        }
    }
}
