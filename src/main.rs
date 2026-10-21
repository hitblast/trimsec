use std::env::args;

use trimsec::args::fetch_noninteractive_ctx_cmd;

#[cfg(feature = "interactive")]
mod interactive;
#[cfg(feature = "interactive")]
use interactive::launch_interactive;

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let eexit = |e, is_error| {
        eprintln!("{}{e}", if is_error { "Error: " } else { "" });
        std::process::exit(1);
    };

    // Interactive Mode
    //
    // NOTE: If trimsec is not compiled with the `interactive` feature,
    // the global help-text will be shown instead.
    if args().len() == 1 {
        #[cfg(feature = "interactive")]
        if let Err(e) = launch_interactive() {
            eexit(e, true)
        } else {
            return;
        }
    }

    // Non-Interactive Mode
    match fetch_noninteractive_ctx_cmd() {
        Ok((mut ctx, cmd)) => {
            if let Err(e) = cmd.run(&mut ctx, None) {
                eexit(e, true);
            }
        }
        Err(e) => {
            eexit(e, false);
        }
    }
}
