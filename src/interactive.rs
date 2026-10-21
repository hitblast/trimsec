use std::io::Write;

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};
use trimsec::{args::parse_deter, core::context::Ctx};

pub fn launch_interactive() -> Result<()> {
    println!(
        "trimsec interactive mode\nversion: {} (type \"exit\" to terminate)",
        env!("CARGO_PKG_VERSION")
    );

    let mut rl = DefaultEditor::new()?;
    let mut ctx = Ctx::new_bare();

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let input: Vec<String> = line.split_whitespace().map(|f| f.to_string()).collect();

                let Some(first) = input.first() else {
                    println!("expected an expression!");
                    continue;
                };

                match first.as_str() {
                    "exit" => return Ok(()),
                    "clear" => {
                        print!("\x1B[2J\x1B[H");
                        std::io::stdout().flush()?;
                    }
                    _ => match parse_deter(&mut ctx, &input, None) {
                        Ok(cmd) => {
                            if let Err(e) = cmd.run(&mut ctx, Some(&input)) {
                                println!("{e}")
                            }
                        }
                        Err(e) => println!("Parsing error: {e}"),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
