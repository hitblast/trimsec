//! Draw cool errors in trimsec using this module!
//!
use std::{
    io::IsTerminal,
    sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering},
    },
};
use terminal_size::terminal_size;

use crate::{args::SESSION_ARGS, style::Style};

/// Returns the spacing before a given argument, within a collection of arguments,
/// when they are represented as a string.
///
/// **Panics:**
///
/// This function can panic if the `idx` overflows within `args`, for which no
/// error-handling procedure has been set up for increasing performance.
fn get_space_for(idx: usize, args: &[String]) -> (usize, usize) {
    let arg_len = args[idx].len();

    if idx == 0 {
        (0, arg_len)
    } else {
        let spacing = args[..idx].iter().map(|s| s.len() + 1).sum();
        (spacing, arg_len)
    }
}

/// Creates a full, pre-initialized space-map for all items in a given argument-collection.
///
/// In trimsec, this function is particularly used for storing the space-map for the program's
/// given arguments (via TTY) for a given session. This is not used for the interactive mode,
/// however, and instead only the [`get_space_for`] function is used.
fn generate_spacing_index(args: &[String]) -> Vec<(usize, usize)> {
    let mut offsets = Vec::with_capacity(args.len());
    let mut current_offset = 0;

    for arg in args {
        let len = arg.len();
        offsets.push((current_offset, len));
        current_offset += len + 1; // length + space
    }

    offsets
}

pub static IS_INTERACTIVE: AtomicBool = AtomicBool::new(false);
static SESSION_ARGS_SPACES: LazyLock<Vec<(usize, usize)>> =
    LazyLock::new(|| generate_spacing_index(&SESSION_ARGS));
static IS_NOTTY: LazyLock<bool> =
    LazyLock::new(|| !std::io::stdin().is_terminal() || IS_INTERACTIVE.load(Ordering::Relaxed));

/// Use this function to draw stuff in the terminal.
pub fn point_at_arg(
    idx: usize,
    args: &[String],
    style: &Style,
    specific_idx: Option<usize>,
    warn: bool,
    msg: Option<&str>,
) {
    let notty = *IS_NOTTY;

    let (spacing, arg_len) = if notty {
        get_space_for(idx, args)
    } else {
        SESSION_ARGS_SPACES[idx]
    };

    let args_joined_len = args.iter().map(|f| f.len() + 1).sum::<usize>() - 1;
    let term_width = terminal_size().map(|f| f.0.0 as usize);

    if let Some(term_width) = term_width
        && args_joined_len + 10 > term_width
    {
        // opinionated numbers, don't worry
        let print_width = (term_width - 20).min(arg_len);
        let arg: &str = &args[idx][0..print_width];

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
            args.join(" "),
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
