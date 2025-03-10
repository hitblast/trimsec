# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.3.1] - 2025.03.05

### Changed

- The handling of the canonical path which is used to store the time bank data, so that it's always absolute (by @hitblast)

## [v0.3.0] - 2025.02.20

### Added

- `trimsec bank` command with the following subcommands: (by @hitblast)
  - `trimsec bank show`
  - `trimsec bank reset`
  - `trimsec bank path`

### Changed

- The CLI workflow - now placed within the `trimsec trim` command (by @hitblast)

### Changed

## [v0.2.0] - 2024.12.27

No noticeable changes.

## [v0.1.9] - 2024.12.26

### Added

- Calculation of the estimated time remaining in current day after trimming (by @hitblast)

### Changed

- Support for calculating even at 1x multiplier (correlate with "Added" section) (by @hitblast)
- "Better?" text output formatting (by @hitblast)

## [v0.1.8] - 2024.12.26

### Added

- `--emoji` flag for using emojis in the output (by @hitblast)

## [v0.1.7] - 2024.12.24

### Added

- `--seconds` flag for using seconds as the default time unita (by @hitblast)
- Automation workflows (GitHub Actions) for releases (by @hitblast)

### Changed

- Quit parsing time internally inside `lib.rs` (by @hitblast)

## [v0.1.6] - 2024.12.24

### Added

- Support for floating-point numbers in the duration argument (by @hitblast)

## [v0.1.5] - 2024.12.24

### Changed

- Fixed misleading documentation (by @hitblast)

## [v0.1.4] - 2024.12.24

### Added

- Graceful exit on invalid input (by @hitblast)
- `-d, --duration-only` flag to only output the new estimated duration (by @hitblast)
- `-t, --time-saved-only` flag to only output the estimated time saved on multiplier usage (by @hitblast)
- Inner CLI optimizations (by @hitblast)

## [v0.1.3] - 2024.12.23

### Added

- Support for trimming multiple durations with same multiplier (by @hitblast)

## [v0.1.2] - 2024.12.23

### Added

- Basic clap integration (by @hitblast)

## [v0.1.1] - 2024.12.22

### Changed

- Make codebase more streamlined (by @furtidev): [View pull request](https://github.com/hitblast/trimsec/pull/1)
- Restructured the markdown entrypoint (by @hitblast)

## [v0.1.0] - 2024.12.22

Initial release.
