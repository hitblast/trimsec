<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

> [!NOTE]
> Want a very simple, basic version of it? Try [trimsec.c](https://gist.github.com/hitblast/3898c05bc13385507a0c37db3b19608e).

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
  - [Enabling YouTube Capabilities](#enabling-youtube-capabilities)
  - [Basic Trimming](#basic-trimming)
  - [Fit-Checking](#fit-checking)
  - [Utility Commands](#utility-commands)
- [Installation](#installation)
- [License](#license)

## Overview

trimsec helps you plan your content consumption. It includes utility functions such as duration-trimming (AKA calculating how much time you'd have in hand _after_ you've watched a video with a multiplier), playlist and video-trimming (same stuff but for YouTube), and other commands. It is still a work-in-progress project, and more commands are already planned!

It is oriented towards terminal-headed academic nerds (pun intended) who can't seem to get a grasp of their syllabus until the last night before the exam, when they already have hundreds of videos to cover. Theoretically impossible to cover - trimsec makes it easier.

## Usage

### 1. Basic Trimming

#### For string-durations:

To calculate saved time, you run the `trim` command as follows:

```bash
ts trim 1h 2x
```

This command outputs the time you saved by watching an hour-long video at 2x the speed. This works for any integer or floating-point combination on either the duration or the multiplier:

```bash
ts trim 1h30m 1.5x
ts trim 1.5h30m 1.5x  # equivalent to 2 hours
```

Combine multiple durations like this:

```bash
ts trim 1h30m+2h50m 1.25x
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
ts trim https://www.youtube.com/watch?v=D4iiKkjGJmU 1.25x
```

You can also throw in a YouTube playlist in (almost) any format you want, and it'd show the total time saved based on the multiplier:

```bash
ts trim "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS" 1.8x
```

For calculating based on the data of only a few items in the playlist, use:

```bash
ts trim <PLAYLIST_URL> --max-items 7 1.8x
```

### Fit-Checking

You can check whether a particular YouTube content fits in a given budget of time like as follows:

```bash
# budget is today
ts fit "https://youtube.com/..."

# shortened param names; budget is 2 hours and 4 minutes
ts fit "https://youtube.com/..." -b 2h4m

# youtube playlist + item cap
ts fit "https://youtube.com/playlist?..." --max-items 5
```

### Utility Commands

- 1. For listing the contents in a YouTube playlist:

```bash
ts list <PLAYLIST_URL>
```

## Installation

### Homebrew

```bash
brew tap hitblast/tap
# Optional (if Homebrew does not trust the tap): brew trust hitblast/tap
brew install trimsec
```

### cargo

```bash
cargo install trimsec
```

### mise

```bash
mise use -g cargo:trimsec
```

### Prebuilt Binaries

Get platform-based binaries here: https://github.com/hitblast/trimsec/releases

### Manual Installation

```bash
git clone https://github.com/hitblast/trimsec.git
cd trimsec && cargo build --release
```

## License

This project is licensed under the [MIT License](LICENSE).
