## Changelog

Active since v2.0.0.

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
