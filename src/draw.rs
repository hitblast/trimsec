use std::{collections::HashMap, io::IsTerminal, sync::LazyLock};
use terminal_size::{Width, terminal_size};

use crate::{args::SESSION_ARGS, style::Style};

static JOINED_SESSION_ARGS: LazyLock<String> = LazyLock::new(|| SESSION_ARGS.join(" "));

static TERM_WIDTH: LazyLock<Option<usize>> = LazyLock::new(|| {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
});

fn generate_spacing_index(args: &Vec<String>) -> HashMap<usize, (usize, usize)> {
    let mut hashed: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut counter: usize = 0;

    for (i, arg) in args.iter().enumerate() {
        for (char_i, _) in arg.chars().enumerate() {
            if char_i == 0 {
                hashed.insert(i, (counter + i, arg.len()));
            }
            counter += 1;
        }
    }

    hashed
}

static SESSION_ARGS_SPACES: LazyLock<HashMap<usize, (usize, usize)>> =
    LazyLock::new(|| generate_spacing_index(&SESSION_ARGS));

pub fn point_at_arg(
    idx: usize,
    args: Option<&Vec<String>>,
    style: &Style,
    specific_idx: Option<usize>,
    warn: bool,
    msg: Option<&str>,
) {
    let (spacing, arg_len) = if let Some(args) = args {
        generate_spacing_index(&args)[&idx]
    } else {
        SESSION_ARGS_SPACES[&idx]
    };

    let cur_args = if let Some(args) = args {
        args
    } else {
        &SESSION_ARGS
    };

    if let Some(w) = *TERM_WIDTH
        && cur_args.len() + 10 > w
    {
        // opinionated numbers, don't worry
        let print_width = (w - 20).min(arg_len);
        let arg: &str = &cur_args[idx][0..print_width];

        println!(
            "\n   {}ts ... {}{}{}{}...{}\n          {}",
            style.grey(),
            style.reset(),
            arg,
            if print_width == arg_len { " " } else { "" },
            style.grey(),
            style.reset(),
            if let Some(s) = specific_idx {
                let x = " ".repeat(s);
                format!(
                    "{}{}^ {}{}",
                    x,
                    style.boldred(),
                    msg.unwrap_or_default(),
                    style.reset()
                )
            } else {
                format!(
                    "{}{} {}{}",
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    "^".repeat(print_width),
                    msg.unwrap_or_default(),
                    style.reset()
                )
            },
        );
    } else {
        let notty = !std::io::stdin().is_terminal() || args.is_some();
        let joined_user_args = args.map(|f| f.join(" "));

        println!(
            "\n   {}{}\n{}   {}{}",
            if notty { "" } else { "ts " },
            if let Some(x) = joined_user_args.as_deref() {
                x
            } else {
                &*JOINED_SESSION_ARGS
            },
            if !notty { "   " } else { "" },
            " ".repeat(spacing),
            if let Some(s) = specific_idx {
                let x = " ".repeat(s);
                format!(
                    "{}{}^ {}{}",
                    x,
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    msg.unwrap_or_default(),
                    style.reset()
                )
            } else {
                format!(
                    "{}{} {}{}",
                    if warn {
                        style.boldgrey()
                    } else {
                        style.boldred()
                    },
                    "^".repeat(arg_len),
                    msg.unwrap_or_default(),
                    style.reset()
                )
            },
        );
    }
}
