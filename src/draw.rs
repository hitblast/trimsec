use std::{collections::HashMap, io::IsTerminal, sync::LazyLock};
use terminal_size::{Width, terminal_size};

use crate::{args::SESSION_ARGS, style::Style};

static JOINED_ARGS: LazyLock<String> = LazyLock::new(|| SESSION_ARGS.join(" "));

static TERM_WIDTH: LazyLock<Option<usize>> = LazyLock::new(|| {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
});

static SPACES: LazyLock<HashMap<usize, (usize, usize)>> = LazyLock::new(|| {
    let mut hashed: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut counter: usize = 0;

    for (i, arg) in SESSION_ARGS.iter().enumerate() {
        for (char_i, _) in arg.chars().enumerate() {
            if char_i == 0 {
                hashed.insert(i, (counter + i, arg.len()));
            }
            counter += 1;
        }
    }

    hashed
});

pub fn point_at_arg(idx: usize, style: &Style, specific_idx: Option<usize>, warn: bool) {
    let (spacing, arg_len) = SPACES[&idx];

    if let Some(w) = *TERM_WIDTH
        && JOINED_ARGS.len() > w
    {
        let print_width = (w - 20).min(arg_len); // opinionated number, don't worry
        let arg: &str = &SESSION_ARGS[idx][0..print_width];

        println!(
            "\n   {}... {}{}{}{}...{}\n       {}",
            style.boldgrey(),
            style.reset(),
            arg,
            if print_width == arg_len { " " } else { "" },
            style.boldgrey(),
            style.reset(),
            if let Some(s) = specific_idx {
                let x = "^".repeat(s);
                format!(
                    "{}{}{}^{}{}{}",
                    style.boldgrey(),
                    x,
                    style.boldred(),
                    style.boldgrey(),
                    x,
                    style.reset()
                )
            } else {
                format!(
                    "{}{}{}",
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    "^".repeat(print_width),
                    style.reset()
                )
            },
        );
    } else {
        let notty = !std::io::stdin().is_terminal();
        println!(
            "\n   {}{}\n{}   {}{}",
            if notty { "" } else { "ts " },
            *JOINED_ARGS,
            if !notty { "   " } else { "" },
            " ".repeat(spacing),
            if let Some(s) = specific_idx {
                let x = "^".repeat(s);
                format!(
                    "{}{}{}^{}{}{}",
                    style.boldgrey(),
                    x,
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    style.boldgrey(),
                    x,
                    style.reset()
                )
            } else {
                format!(
                    "{}{}{}",
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    "^".repeat(arg_len),
                    style.reset()
                )
            },
        );
    }
}
