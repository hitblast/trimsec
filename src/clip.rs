use std::{
    fmt::Display,
    ops::Deref,
    sync::{LazyLock, Mutex},
};

use crate::errors::FetchTClipStringError;

pub static CLIPBOARD: LazyLock<Result<Mutex<arboard::Clipboard>, String>> = LazyLock::new(|| {
    arboard::Clipboard::new()
        .map(Mutex::new)
        .map_err(|e| e.to_string())
});

/// A special type of `String` which inherits itself from the clipboard.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TClipString {
    inner: String,
}

impl TClipString {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.inner
    }

    pub fn resolve(s: &str) -> Result<Option<Self>, FetchTClipStringError> {
        if s == "-clip" {
            match &*CLIPBOARD {
                Ok(clip) => {
                    let mut clip = clip.lock().map_err(|e| {
                        FetchTClipStringError::ClipboardMutexPoisoned(e.to_string())
                    })?;
                    let content = clip.get_text().map_err(|e| {
                        FetchTClipStringError::ClipboardContentFetchFailed(e.to_string())
                    })?;

                    Ok(Some(TClipString { inner: content }))
                }
                Err(e) => Err(FetchTClipStringError::Clipboard(e.clone())),
            }
        } else {
            Ok(None)
        }
    }
}

impl Display for TClipString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for TClipString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for TClipString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl From<String> for TClipString {
    fn from(s: String) -> Self {
        Self { inner: s }
    }
}

impl From<&str> for TClipString {
    fn from(s: &str) -> Self {
        Self {
            inner: s.to_string(),
        }
    }
}

impl From<TClipString> for String {
    fn from(s: TClipString) -> Self {
        s.inner
    }
}
