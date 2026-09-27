use trimsec::args::get_cur_cmd;

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

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
