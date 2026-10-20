use std::{collections::HashMap, io::IsTerminal, sync::LazyLock};
use terminal_size::{Width, terminal_size};

use crate::{
    args::{JOINED_SESSION_ARGS, SESSION_ARGS},
    style::Style,
};

static TERM_WIDTH: LazyLock<Option<usize>> = LazyLock::new(|| {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
});

/// Returns the spacing before a given argument, within a collection of arguments,
/// when they are represented as a string.
///
/// **Panics:**
///
/// This function can panic if the `idx` overflows within `args`, for which no
/// error-handling procedure has been set up for increasing performance.
fn get_space_for(idx: usize, args: &Vec<String>) -> (usize, usize) {
    if idx == 0 {
        (0, args[0].len())
    } else {
        (args[..idx].join(" ").len() + 1, args[idx].len())
    }
}

/// Creates a full, pre-initialized space-map for all items in a given argument-collection.
///
/// In trimsec, this function is particularly used for storing the space-map for the program's
/// given arguments (via TTY) for a given session. This is not used for the interactive mode,
/// however, and instead only the [`get_space_for`] function is used.
fn generate_spacing_index(args: &Vec<String>) -> HashMap<usize, (usize, usize)> {
    let mut hashed: HashMap<usize, (usize, usize)> = HashMap::new();

    for i in 0..args.len() {
        hashed.insert(i, get_space_for(i, args));
    }

    hashed
}

static SESSION_ARGS_SPACES: LazyLock<HashMap<usize, (usize, usize)>> =
    LazyLock::new(|| generate_spacing_index(&SESSION_ARGS));

/// Use this function to draw stuff in the terminal.
pub fn point_at_arg(
    idx: usize,
    args: Option<&Vec<String>>,
    style: &Style,
    specific_idx: Option<usize>,
    warn: bool,
    msg: Option<&str>,
) {
    let ((spacing, arg_len), cur_args, joined_cur_args, term_width, notty) =
        if let Some(args) = args {
            (
                get_space_for(idx, args),
                args,
                args.join(" "),
                terminal_size().map(|f| f.0.0 as usize),
                true,
            )
        } else {
            (
                SESSION_ARGS_SPACES[&idx],
                &*SESSION_ARGS,
                JOINED_SESSION_ARGS.clone(),
                *TERM_WIDTH,
                !std::io::stdin().is_terminal(),
            )
        };
    let joined_cur_args = &joined_cur_args;

    if let Some(term_width) = term_width
        && joined_cur_args.len() + 10 > term_width
    {
        // opinionated numbers, don't worry
        let print_width = (term_width - 20).min(arg_len);
        let arg: &str = &cur_args[idx][0..print_width];

        println!(
            "\n   {}{}... {}{}{}{}...{}\n{}       {}",
            style.grey(),
            if notty { "" } else { "ts " },
            style.reset(),
            arg,
            if print_width == arg_len { " " } else { "" },
            style.grey(),
            style.reset(),
            if notty { "" } else { "   " },
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
        println!(
            "\n   {}{}\n{}   {}{}",
            if notty { "" } else { "ts " },
            joined_cur_args,
            if notty { "" } else { "   " },
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
