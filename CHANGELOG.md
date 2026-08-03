## Changelog

Active since v2.0.0.

### v2.5.1

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
