use std::{collections::HashMap, env, sync::LazyLock};

use terminal_size::{Width, terminal_size};

use crate::style::Style;

static RAW_ARGS: LazyLock<Vec<String>> = LazyLock::new(|| env::args().collect());
static JOINED_ARGS: LazyLock<String> = LazyLock::new(|| RAW_ARGS.join(" "));

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

    for (i, arg) in RAW_ARGS.iter().enumerate() {
        for (char_i, _) in arg.chars().into_iter().enumerate() {
            if char_i == 0 {
                hashed.insert(i, (counter + i, arg.len()));
            }
            counter += 1;
        }
    }

    hashed
});

pub fn point_at_arg(mut idx: usize, style: &Style, specific_idx: Option<usize>) {
    idx += 1;
    let (spacing, arg_len) = SPACES[&idx];

    if let Some(w) = *TERM_WIDTH
        && JOINED_ARGS.len() > w as usize
    {
        let print_width = (w - 20).min(arg_len);
        let arg: &str = &RAW_ARGS[idx][0..print_width];
        println!(
            "\n{}... {}{}{}{}...{}\n    {}{}{}",
            style.grey(),
            style.reset(),
            arg,
            if print_width == arg_len { " " } else { "" },
            style.grey(),
            style.reset(),
            style.boldred(),
            "^".repeat(print_width),
            style.reset()
        );
    } else {
        println!(
            "\n{}\n{}{}",
            *JOINED_ARGS,
            " ".repeat(spacing),
            if let Some(s) = specific_idx {
                format!(
                    "{}{}{}^{}{}{}",
                    style.boldgrey(),
                    "^".repeat(s),
                    style.boldred(),
                    style.boldgrey(),
                    "^".repeat(s),
                    // "^".repeat(arg_len - s - 1),
                    style.reset()
                )
            } else {
                format!(
                    "{}{}{}",
                    style.boldred(),
                    "^".repeat(arg_len),
                    style.reset()
                )
            },
        );
    }
}
