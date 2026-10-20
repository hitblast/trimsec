use std::{
    env::args,
    io::{self, Write},
};

use anyhow::Result;
use trimsec::{
    args::{fetch_ctx_and_runnable, parse_deter},
    core::context::Ctx,
};

fn launch_interactive() -> Result<()> {
    let grab_input = || -> Result<Vec<String>> {
        print!(">>> ");
        io::stdout().flush()?;

        let mut buf: String = String::new();
        io::stdin()
            .read_line(&mut buf)
            .map_err(|e| anyhow::anyhow!("Interactive input error: {e}"))?;

        Ok(buf
            .split_whitespace()
            .map(|f| f.to_string())
            .collect::<Vec<String>>())
    };

    let mut ctx = Ctx::new_bare();

    loop {
        let input = grab_input()?;

        let Some(first) = input.first() else {
            println!("expected an expression!");
            continue;
        };

        match first.as_str() {
            "exit" | "quit" => return Ok(()),
            _ => {
                let cmd = parse_deter(&mut ctx, &input, None)?;

                if let Err(e) = cmd.run(&mut ctx) {
                    println!("{e}")
                }
            }
        }
    }
}

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let eexit = |e, is_error| {
        eprintln!("{}{e}", if is_error { "Error: " } else { "" });
        std::process::exit(1);
    };

    if args().len() == 1 {
        if let Err(e) = launch_interactive() {
            eexit(e, true)
        }
    } else {
        match fetch_ctx_and_runnable() {
            Ok((mut ctx, cmd)) => {
                if let Err(e) = cmd.run(&mut ctx) {
                    eexit(e, true);
                }
            }
            Err(e) => {
                eexit(e, false);
            }
        }
    }
}
