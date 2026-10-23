use trimsec::{
    args::{SESSION_ARGS, TKeywordArgs, fetch_noninteractive_cmd},
    core::context::Ctx,
};

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

    match TKeywordArgs::parse(&SESSION_ARGS) {
        Ok((kwargs, remaining, skippable)) => {
            let mut ctx = Ctx::new(kwargs);

            #[cfg(feature = "interactive")]
            if remaining.len() == 0 {
                if let Err(e) = launch_interactive(&mut ctx) {
                    eexit(e, true)
                } else {
                    return;
                }
            }

            match fetch_noninteractive_cmd(&mut ctx, remaining, skippable) {
                Ok(cmd) => {
                    if let Err(e) = cmd.run(&mut ctx, None) {
                        eexit(e, true);
                    }
                }
                Err(e) => {
                    eexit(e, false);
                }
            }
        }
        Err(e) => eexit(e, true),
    }
}
