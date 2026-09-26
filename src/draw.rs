use std::{collections::HashMap, env, sync::LazyLock};

use crate::style::Style;

static RAW_STR_ARGS: LazyLock<Vec<String>> = LazyLock::new(|| env::args().collect());

static SPACES: LazyLock<HashMap<usize, (usize, usize)>> = LazyLock::new(|| {
    let mut hashed: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut counter: usize = 0;

    for (i, arg) in RAW_STR_ARGS.iter().enumerate() {
        for (char_i, _) in arg.chars().into_iter().enumerate() {
            if char_i == 0 {
                hashed.insert(i, (counter + i, arg.len()));
            }
            counter += 1;
        }
    }

    hashed
});

pub fn draw_args_arrow(mut idx: usize, style: &Style) {
    idx += 1;
    let (spacing, arg_len) = SPACES[&idx];

    println!(
        "{}\n{}{}{}{}",
        RAW_STR_ARGS.join(" "),
        " ".repeat(spacing),
        style.boldred(),
        "^".repeat(arg_len),
        style.reset()
    );
}
