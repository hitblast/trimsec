cargo clippy --all-targets --no-deps -v -- -D warnings
cargo fmt --all -- --check
cargo test
