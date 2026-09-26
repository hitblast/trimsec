<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

![Crates.io Total Downloads](https://img.shields.io/crates/d/trimsec?color=black)

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
  - [Trimming](#1-trimming)
  - [Fit-Check](#2-fit-check)
  - [Utility Commands](#3-utility-commands)
- [Configuration](#configuration)
  - [Configuration Options](#configuration-options)
- [Installation](#installation)
- [License](#license)

## Overview

trimsec helps you plan your content consumption with two primary functions:

1. Fit-checking contents for the remainder of the day, or a specific time-budget, and
2. Trimming the content using multipliers and calculating how much time you'll save.

Everything else is cherry-on-top.

## Usage

### 1. Trimming

To calculate saved time, you run a pattern like this:

```bash
# duration and multiplier used
ts 1h 2x
```

trimsec can receive an arbitrary amount of inputs and can output trims based on that:

```bash
ts 1h30m 1.5x 3h 2x
# or
ts 1h30m 1.5x 1.2x 3h 2m
# or even
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

You can also prefix the playlist URL with `max:<n>::` to only calculate for the first `n` elements of the playlist:

```bash
# only the first 5 items
ts 1.8x "max:5::https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
```

### 2. Fit-Check

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

### 3. Utility Commands

trimsec also contains utility commands coherent with the two primary features to help you do more:

- `ts list <URL>`: List contents in a YouTube playlist.

(more to be added)

## Configuration

The config file for trimsec lies in the home directory of the user:

- Linux: `$HOME/.trimsecrc` (e.g. `/home/alice/.trimsecrc`)
- macOS: `$HOME/.trimsecrc` (e.g. `/Users/hitblast/.trimsecrc`)
- Windows: `C:\Users\<username>\.trimsecrc`

The primary use of the config file is to store the [API key (see "For YouTube videos/playlists")](#for-youtube-videosplaylists) for Google Cloud Console, and to store configuration options for trimsec itself, which are described below.

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

- Or,

Get platform-based binaries here: https://github.com/hitblast/trimsec/releases

### Manual Installation

```bash
git clone https://github.com/hitblast/trimsec.git
cd trimsec && cargo build --release
```

## License

This project is licensed under the [MIT License](LICENSE).
