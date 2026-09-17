## Changelog

Active since v2.0.0.

### v2.9.0

New features:

- Configuration options have been added (a new `[options]` table in .trimsecrc):
  - `default_multiplier`: The default multiplier to use for trimming operations.

### v2.8.0

New features:

Changes:

- Added `fits` as an alias for the `fit` (previously `fits`) command.
- Removed `clap_complete` from the regular dependency tree.

Internal changes:

- The `Runnable` trait for commands now, instead of passing in style and other attributes to the commands which will never use them, passes a new `Ctx` object which is a simple context lazy-loader. It holds `Style` and `Config` for now.
- The new `Ctx` also changes some function signatures for the internal API.
- `Style` is now under `crate::commands::style`, although this might not be its final destination.
- `crate::core::api::client`:
  - URLs are now formed using a new `gen_url!()` macro and via the `.query_pairs()` method of `ureq`'s `RequestBuilder` struct.
- `crate::core::time`:
  - `TDuration` now manually implements `PartialOrd` instead of using the derive implementation.
- Fixed some `clippy` warnings.
- Changed the names of some internal datatypes.

### v2.7.2

Changes:

- The `trim` command now skips processing altogether if the `multiplier` that's passed in turns out to be `1.0`.
- Made some parity changes towards all the `max_items` arguments across all commands.

### v2.7.1

Bug fixes:

- Fixed a bug which led to the trimming logic falling apart internally while executing the `TrimCmd::yt_fallback` function if the `trim` command was executed with `--max-items`, as `TrimCmd::run` used to stop if the passed content was a duration and _also_ had a nonzero `max_items` at the same time.

### v2.7.0

New additions:

- Added a new `cut` alias for the `trim` command.
- Both the `yt` and `trim` commands are now combined into one `trim` command.
- The internal API has been improved drastically, with trait implementations for various default data types for conversation, as well as syntactical sugar for displaying durations, and more.

### v2.6.0

Bug fixes:

- Fixed a bug in `key set` and the general config-loading sequence to always ensure a fresh config file (create a new one if the file did not exist already).

New additions:

- Changed command `fits` to `fit`.

Changes:

- Removed clipboard functionality. Users can just paste it.
- Removed the global `--clip` flag.
- Prioritized using positional arguments instead of keyword arguments for all commands.
- Changed the internal output type for a lot of functions from `(f64, i64)` to a new `TDuration` type, which holds `seconds()` as an f64 value and `splits()` as an `u64`.

### v2.5.2

New features:

- Divided the previous `ts key` command into a command "group":
  - `ts key set` now sets the API key.
  - `ts key show` _shows_ the existing API key (if any, otherwise shows "not set").
- Used the `ureq` and `url` crates respectively for HTTP requests and URL parsing, and removed `reqwest` from the dependency tree.
- Also removed `serde_json` from the dependency tree as no longer needed (`ureq` has built-in JSON parsing via the `json` feature).

### v2.5.1

New features:

- Added: `ts path`

Changes:

- The layout for `ts fits` now matches that of `ts trim` or `ts yt`.

Internal changes:

- Improvements to the styling API have been made.

### v2.5.0

New features:

- Add support for `music.youtube.com` URLs.
- Added `ts key --show`.

Breaking changes:

- Primary arguments are now positional by design.
- `ts trim` now accepts the multiplier as a keyword-only `multiplier` argument to keep it unified across commands.
- Changed the `fitcheck` command to `fits`.

Bug fixes:

- Fixed `ts fitcheck` (now `fits`) not working at all.
- Overridden some usage for some commands to fix some issues caused by the new positional arguments.

### v2.4.0

New features:

- Added a new `key`command to set the API key easily.
- Added new aliases for a handful of commands:
  - `yt` -> changed from `vid` to `y`
  - `list` -> `ls`

Changes:

- Unknown fields are now prohibited in the config file.
- Removed `--choose` flag from `ts fitcheck`.
- Replaced the global `--no-clip` argument with `clip`, and made all `link` arguments across the command palette required (unless `clip` is present).
- The CLI `--link` argument, for commands that include it, is prioritized internally over the clipboard (when `--clip` is passed). Although one of them will get cancelled out anyway, this is more of a _redundancy_ change.

### v2.3.0

New features:

- Added `ts fitcheck` / `ts fc` (notes included in README.md).
- Added `ts list` (lists all YouTube contents to stdin).
- Added support for a `.trimsecrc` file which receives a TOML file as input (for later use in settings + storing the API key).
- Added a new, global `--color` value enum.
- trimsec now obeys the `NO_COLOR` standard.

Changes:

- Running `trim` (or `yt`, which uses `trim` underneath) should no longer view the "Time in day left: <x>s" string, if there's no time left at all (`0s`).
- `-n` / `--no-clip` is a universal flag now.
- Reduced `println!()` calls across commands without hampering the output.
- Internal code changes (reduced duplicate code through the use of `youtils.rs` and `utils.rs` modules).

### v2.2.1

New features:

- Added `-n`/`--no-clip` (to `ts yt`) for intentionally disabling the clipboard-fetcher.

Changes:

- Improved some error descriptions.

### v2.2.0

New features:

- `max_items` is now limited by an internal limit of maximum items, which is fetched when a playlist is passed as an arguement, so that the user never "overshoots" the length manually.
- New error variants to clarify better.

Internal changes:

- More code optimizations and stuff.
- Removed the `Runnable` trait as it is not used for dispatch changes at all.
- Removed yet another `.unwrap()` from the codebase.

Fixes:

- Fixed the usage guide for `ts yt`, _again_.

### v2.1.1

Fixes:

- Fixed usage for `ts yt`.

### v2.1.0

New features:

- Running `ts yt` with a playlist URL now also outputs the amount of items (from the playlist) which were traversed.

Breaking changes:

- Removed `-c`/`--clip` - CLI now defaults to clipboard if `link` is not found.

Internal changes:

- Added the `/playlist` URL path to the `get_youtube_id` function's scope.
- Overridden usage documentation for `ts yt`.

### v2.0.0

New features:

- Add command: `ts yt` (for YouTube-related trimming functionality)

Internal changes:

- Internal optimizations and code structure changes for a proper CLI feel (use `clap` and `anyhow` for CLI-side structuring and error management).
