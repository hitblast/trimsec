<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
  - [Basic Trimming](#basic-trimming)
  - [Fit-Checking](#fit-checking)
  - [Utility Commands](#utility-commands)
- [Configuration](#configuration)
  - [Configuration Options](#configuration-options)
- [Installation](#installation)
- [License](#license)

## Overview

trimsec helps you plan your content consumption.

It helps you trim your planned content using multipliers, check if it fits within a given duration or the remainder of the day, and do much more. trimsec supports YouTube data via Google's APIs, so you can even execute these operations for your online videos. It is oriented towards terminal-headed academic nerds who can't seem to get a grasp of their syllabus until the last night before the exam.

trimsec will always be a work-in-progress.

## Usage

### 1. Basic Trimming

#### For string-durations:

To calculate saved time, you run the `trim` command as follows:

```bash
ts 1h 2x
```

This command outputs the time you saved by watching an hour-long video at 2x the speed. This works for any integer or floating-point combination on either the duration or the multiplier:

```bash
ts 1h30m 1.5x
ts 1.5h30m 1.5x  # equivalent to 2 hours
```

Combine multiple durations like this:

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

In place of the duration from before, now you just paste the YouTube video URL:

```bash
ts https://www.youtube.com/watch?v=D4iiKkjGJmU 1.25x
```

You can also throw in a YouTube playlist in (almost) any format you want, and it'd show the total time saved based on the multiplier:

```bash
ts "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS" 1.8x
```

For calculating based on the data of only a few items in the playlist, use:

```bash
ts <PLAYLIST_URL> --max-items=7 1.8x
```

### 2. Fit-Checking

You can check whether a particular YouTube content fits in a given budget of time like as follows:

```bash
# budget is today
ts 1h24m
ts "https://youtube.com/..."

# fixed duration: 2 hours and 4 minutes
ts 1h24m 2h4m
ts "https://youtube.com/..." 2h4m

# youtube playlist + item cap
ts "https://youtu.be/..." --max-items=5
```

### 3. Utility Commands

- For listing the contents in a YouTube playlist:

```bash
ts list <PLAYLIST_URL>  # or: ts ls <PLAYLIST_URL>
```

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
