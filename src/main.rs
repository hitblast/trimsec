use trimsec::args::fetch_ctx_and_runnable;

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let eexit = |e, is_error| {
        eprintln!("{}{e}", if is_error { "Error: " } else { "" });
        std::process::exit(1);
    };

    match fetch_ctx_and_runnable() {
        Ok((ctx, cmd)) => {
            if let Err(e) = cmd.run(ctx) {
                eexit(e, true);
            }
        }
        Err(e) => {
            eexit(e, false);
        }
    }
}
