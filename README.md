<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

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

trimsec helps you plan your content consumption via two primary features:

1. Fitting if contents fit into the remaining day, or an explicit timeframe you give, and
2. Trimming the content using multipliers and calculating how much time you'll save.

Everything else is cherry-on-top.

## Usage

### 1. Trimming

To calculate saved time, you run trimsec in this pattern:

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

Nearby durations based on the cursor are combined and checked against their common multipliers. This allows checking a huge number of durations in a matter of seconds.

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

trimsec can also process YouTube URLs and trim videos just like trimming regular durations:

```bash
ts "https://www.youtube.com/watch?v=D4iiKkjGJmU" 1.25x
```

Since it invokes the original [trimming function](#1-trimming) underneath, you can also combine durations, and provide an infinite amount of such arguments:

```bash
ts <URL> 1h30m 1.25x 2h <URL> 2x
```

YouTube playlist URLs are also supported and can be combined with other durations too if needed:

```bash
ts 1.8x "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS"
```

### 2. Fit-Check

You can check whether a particular YouTube content fits within a budget using this pattern:

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

Combination rules explained in [1. Trimming](#1-trimming) apply here as well, so other combinations of arguments are also possible outside of this collection.

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
