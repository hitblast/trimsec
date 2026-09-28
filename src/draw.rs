use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

use terminal_size::{Width, terminal_size};

use crate::style::Style;

pub static RAW_ARGS: OnceLock<Vec<String>> = OnceLock::new();
static JOINED_ARGS: LazyLock<String> = LazyLock::new(|| RAW_ARGS.get().unwrap().join(" "));

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

    #[allow(clippy::unwrap_used)]
    for (i, arg) in RAW_ARGS
        .get()
        .expect("could not get entries in raw args for the spaces index")
        .iter()
        .enumerate()
    {
        for (char_i, _) in arg.chars().into_iter().enumerate() {
            if char_i == 0 {
                hashed.insert(i, (counter + i, arg.len()));
            }
            counter += 1;
        }
    }

    hashed
});

pub fn point_at_arg(mut idx: usize, style: &Style, specific_idx: Option<usize>, warn: bool) {
    idx += 1;
    let (spacing, arg_len) = SPACES[&idx];

    if let Some(w) = *TERM_WIDTH
        && JOINED_ARGS.len() > w
    {
        let print_width = (w - 20).min(arg_len); // opinionated number, don't worry

        #[allow(clippy::unwrap_used)]
        let arg: &str = &RAW_ARGS
            .get()
            .expect("could not get entries in raw args for drawing")[idx][0..print_width];

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
        println!(
            "\n   {}\n   {}{}",
            *JOINED_ARGS,
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
