# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
