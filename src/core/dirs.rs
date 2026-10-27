use std::{fs, path::PathBuf, sync::LazyLock};

pub static CACHE_DIR: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    dirs::cache_dir().and_then(|f| {
        let path = f.join("trimsec");
        fs::create_dir_all(&path).ok().map(|_| path)
    })
});
