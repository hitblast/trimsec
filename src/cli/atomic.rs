use std::sync::atomic::{AtomicBool, Ordering};

static ACCEPT_ALL: AtomicBool = AtomicBool::new(false);

pub fn set_accept_all(value: bool) {
    ACCEPT_ALL.store(value, Ordering::SeqCst);
}

pub fn should_accept_all() -> bool {
    ACCEPT_ALL.load(Ordering::SeqCst)
}

static QUIET: AtomicBool = AtomicBool::new(false);

pub fn set_quiet(value: bool) {
    QUIET.store(value, Ordering::SeqCst);
}

pub fn should_be_quiet() -> bool {
    QUIET.load(Ordering::SeqCst)
}

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(value: bool) {
    VERBOSE.store(value, Ordering::SeqCst);
}

pub fn should_be_verbose() -> bool {
    VERBOSE.load(Ordering::SeqCst)
}

static DRY_RUN: AtomicBool = AtomicBool::new(false);

pub fn set_dry_run(value: bool) {
    DRY_RUN.store(value, Ordering::SeqCst);
}

pub fn should_dry_run() -> bool {
    DRY_RUN.load(Ordering::SeqCst)
}
