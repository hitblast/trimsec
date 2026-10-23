use std::{io::Write, path::PathBuf};

use anyhow::{Result, bail};
use rustyline::{DefaultEditor, Editor, error::ReadlineError, history::FileHistory};
use trimsec::{args::parse_deter, core::context::Ctx};

fn get_hist_path() -> Option<PathBuf> {
    if let Some(home_dir) = dirs::home_dir() {
        Some(home_dir.join(".trimsechist"))
    } else {
        None
    }
}

fn load_hist(rl: &mut Editor<(), FileHistory>) -> Result<()> {
    if let Some(p) = get_hist_path() {
        if p.exists() {
            rl.load_history(&p)?;
        }
        Ok(())
    } else {
        bail!("WARN: History file could not be loaded, proceeding without loading.")
    }
}
fn save_hist(rl: &mut Editor<(), FileHistory>) {
    if let Some(p) = get_hist_path() {
        let _ = rl.save_history(&p);
    }
}

pub fn launch_interactive(ctx: &mut Ctx) -> Result<()> {
    println!(
        "trimsec interactive mode\nversion: {} (type \"exit\" to terminate)",
        env!("CARGO_PKG_VERSION")
    );

    let mut rl = DefaultEditor::new()?;

    let mut use_history = ctx
        .config()
        .map(|f| f.interactive_options().use_history())
        .unwrap_or(true);

    if use_history {
        if let Err(e) = load_hist(&mut rl) {
            use_history = false;
            eprintln!("Loading history failed: {e}")
        }
    } else {
        println!("Not using history.")
    }

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if use_history {
                    save_hist(&mut rl);
                }

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
                    _ => match parse_deter(ctx, &input, None) {
                        Ok(cmd) => {
                            if let Err(e) = cmd.run(ctx, Some(&input)) {
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
