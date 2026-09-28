<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

![Crates.io Total Downloads](https://img.shields.io/crates/d/trimsec?color=black)

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
  - [Trimming](#trimming)
  - [Fit-Check](#fit-check)
  - [Explicit Commands](#explicit-commands)
  - [Features](#features)
- [Configuration](#configuration)
  - [Configuration Options](#configuration-options)
- [Installation](#installation)
- [License](#license)

## Overview

trimsec helps you plan your content consumption with two primary functions:

1. Fit-checking contents for the remainder of the day, or a specific time-budget, and
2. Trimming the content using multipliers and calculating how much time you'll save.

Everything else is cherry-on-top.

Development happens on [Codeberg](https://codeberg.org/hitblast/trimsec) and merged onto [GitHub](https://github.com/hitblast/trimsec) for deploying to the release pipeline.

## Usage

### Trimming

To calculate saved time, you run a pattern like this:

```bash
ts 1h 2x
```

trimsec can receive an arbitrary amount of inputs and can output trims based on that:

```bash
ts 1h30m 1.5x 3h 2x
ts 1h30m 1.5x 1.2x 3h 2m
ts 3x 2h 2h 2h 1.25x 1h 2x 3.5h
```

Nearby durations based on the cursor are combined and checked against their common multipliers. This allows checking a huge number of durations in a matter of seconds. trimsec will daisy-chain these commands and also output how much time you'll save by the end of the day/budget.

You can also explicitly combine two durations like this:

```bash
ts 1h30m+2h50m 1.25x
```

#### For YouTube videos/playlists:

> [!NOTE]
> You must set your [Google Cloud Console](https://console.cloud.google.com/) API key first for the **YouTube Data API (v3)**:
>
> ```bash
> ts key set <API_KEY_HERE>
> ```

trimsec can also receive YouTube URLs as arguments alongside all the other durations and multipliers:

```bash
ts "https://www.youtube.com/watch?v=D4iiKkjGJmU" 1.25x
```

This leads to an infinite amount of combinations and daisy-chaining that you can do:

```bash
ts <URL> 1h30m 1.25x 2h <URL> 2x
```

YouTube playlist URLs are also supported and can be combined with other durations too if needed:

```bash
ts 1.8x "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
```

You can also prefix the playlist URL with `max:<n>::` to only calculate for the first `n` elements of the playlist. Here we demonstrate this by grabbing the first 5:

```bash
ts 1.8x "max:5::https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
```

### Fit-Check

For fit-checking, you may use the patterns mentioned below.

Combination rules explained in [1. Trimming](#1-trimming) apply here as well, so other combinations of arguments are also possible outside of this collection.

```bash
# budget is today
ts 1h24m
ts "https://youtube.com/..."

# fixed budget: 2 hours and 4 minutes
ts 1h24m 2h4m
ts "https://youtube.com/..." 2h4m

# two items, explicit budget of 2h4m
ts 1h24m 15m b2h4m

# multiple args, budget is today
ts <URL> 1h30m 2h <URL> 3h

# multiple args, explicit budget of 15h
ts <URL> 1h30m 2h <URL> 3h b15h

# youtube playlist, budget of 5h
ts "https://youtu.be/..." 5h

# youtube playlist, item cap, and other content
ts "max:5::https://youtu.be/..." 3h20m b5h
```

### Explicit Commands

trimsec uses the hybrid of a deterministic parser and [clap](https://github.com/clap-rs/clap), so explicit commands are also possible just like any typical command-line interface.

Some notable commands are:

- `ts list <PLAYLIST_URL>` - Lists the contents of a YouTube playlist.

A few configuration-related commands are:

- `ts path` - Shows the path to the configuration file.
- `ts key <subcommand>` (used for Google Cloud Console API configuration for YouTube support)
  - `ts key show` - Shows the configured API key.
  - `ts key set <API_KEY>` - Sets a new API key.
  - `ts key unset` - Unsets the configured API key.

These are the most commonly-used ones. For viewing all of the commands, run `ts help` and see "Commands".

> [!NOTE]
> The deterministic parser will most likely have different grammar from clap's logic. For example, setting an item-cap for trimming would look something like `ts "max::10::https://youtube.com/..."`, but for the `ts list` command, it would look like `ts list --max-items 10 "https://youtube.com/...`.

### Features

> [!NOTE]
> This is an expanding list as of now.

Pipes are supported just like any other well-built Unix/Linux command-line interface.

```bash
echo "1h2m 1.25x" | ts 2h 3x
```

## Configuration

The configuration file for trimsec lies in the home directory of the user:

- Linux: `$HOME/.trimsecrc` (e.g. `/home/alice/.trimsecrc`)
- macOS: `$HOME/.trimsecrc` (e.g. `/Users/hitblast/.trimsecrc`)
- Windows: `C:\Users\<username>\.trimsecrc`

The primary use of the configuration file is to store the [API key (see "For YouTube videos/playlists")](#for-youtube-videosplaylists) for Google Cloud Console, and to store configuration options for trimsec itself, which are described below.

### Configuration Options

trimsec provides partial modifications of its features through the `options` table.

For example, if you need to set a default time-budget for fit-checks, you can use:

```toml
[options]
default_fit_budget = "2h4m"
```

## Installation

- Using [Homebrew](https://brew.sh):

```bash
brew tap hitblast/tap
# Optional (if Homebrew does not trust the tap): brew trust hitblast/tap
brew install trimsec
```

- Using `cargo`:

```bash
cargo install trimsec
```

- Using [mise](https://github.com/jdx/mise):

```bash
mise use -g cargo:trimsec
```

- Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

  macOS:

  ```bash
  cargo-binstall \
          --pkg-url=https://github.com/hitblast/trimsec/releases/latest/download/trimsec-macos-latest.tar.gz \
          --pkg-fmt=tgz \
          trimsec
  ```

  Linux (x86_64):

  ```bash
  cargo-binstall \
          --pkg-url=https://github.com/hitblast/trimsec/releases/latest/download/trimsec-ubuntu-latest.tar.gz \
          --pkg-fmt=tgz \
          trimsec
  ```

  Windows:

  ```bash
  cargo-binstall \
          --pkg-url=https://github.com/hitblast/trimsec/releases/latest/download/trimsec-win-latest.zip \
          --pkg-fmt=zip \
          trimsec
  ```

- Or,

Get platform-based binaries here: https://github.com/hitblast/trimsec/releases

### Manual Installation

```bash
git clone https://github.com/hitblast/trimsec.git
cd trimsec && cargo build --release
```

## License

This project is licensed under the [MIT License](LICENSE).
