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

### _**(Optional)** Enabling YouTube Capabilities_

> [!NOTE]
> This section is only needed if you're going to be passing in YouTube playlists/videos as arguments.

Steps:

- **Get your API key for the YouTube Data API (v3) from the [Google Cloud Console](https://console.cloud.google.com/)**.
- Run the following command. Make sure to replace `<YOUR_API_KEY>` with your actual API key:

  ```bash
  ts key <YOUR_API_KEY>
  ```

  - Opt 1: Create a new `.trimsecrc` file at your `$HOME` (on Windows: `C:\Users\<USERNAME>`) directory with this structure:

    ```toml
    api_key = "YOUR_API_KEY_HERE"
    ```

    You may easily get the running location of the path using: `ts path`

  - Opt 2: Set it using the `TRIMSEC_YOUTUBE_KEY` environment variable.
    - A good choice is to use [direnv](https://github.com/direnv/direnv) and create a `.envrc` file in your `$HOME`, then run `direnv allow` in your home directory from the terminal to set it as an isolated environment variable, although beware that the variable won't be available in any other directories.

### 1. Basic Trimming

#### For eyeballed durations:

To calculate saved time, you run the `trim` command as follows:

```bash
ts trim 1h -m 2x
```

This command outputs the time you saved by watching an hour-long video at 2x the speed. This works for any integer or floating-point combination on either the duration or the multiplier:

```bash
ts trim 1h30m -m 1.5x
ts trim 1.5h30m -m 1.5x  # equivalent to 2 hours
```

Combine multiple durations like this:

```bash
ts trim 1h30m+2h50m -m 1.25x
```

#### For YouTube videos/playlists:

Instead of using the `trim` command, you'll be using the `yt` (or `y`) command:

```bash
ts yt https://www.youtube.com/watch?v=D4iiKkjGJmU -m 1.25x
```

You can also throw in a YouTube playlist in (almost) any format you want, and it'd show the total time saved based on the multiplier:

```bash
ts yt "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS" -m 1.8x
```

As you can see, the link strings are getting quite _big_. To solve this, you can instead, grab the link from the clipboard:

```bash
ts yt -m 1.8x --clip
```

For traversing only a _certain amount_ of items in a playlist (starting from the 1st item), use this:

```bash
ts yt --max-items 7 "https://youtube.com/..." -m 1.8x
```

### Fit-Checking

You can check whether a particular YouTube content fits in a given budget of time like as follows:

```bash
# link grabbed from clipboard; budget is today
ts fits --clip

# link pasted manually; budget is today
ts fits "https://youtube.com/..."

# shortened param names; budget is 2 hours and 4 minutes
ts fits "https://youtube.com/..." -b 2h4m

# youtube playlist + item cap
ts fits "https://youtube.com/playlist?..." --max-items 5
```

### Utility Commands

> [!NOTE]
> More such commands will be added with the following releases, as these are proportional to ideas coming in.

- For listing the contents in a YouTube playlist:

```bash
ts list "https://youtube.com/..."
ts ls --clip            # shorter; grabs from clipboard
ts ls --clip --max-items 5     # only traverses 5 items
```

## Installation

### Homebrew

```bash
brew tap hitblast/tap && brew trust hitblast/tap
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
