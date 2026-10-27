use std::{path::PathBuf, sync::LazyLock};

pub static CACHE_DIR: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| dirs::cache_dir().map(|f| f.join("trimsec")));
