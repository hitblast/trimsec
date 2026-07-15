<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

> [!NOTE]
> Want a very simple, basic version of it? Try [trimsec.c](https://gist.github.com/hitblast/3898c05bc13385507a0c37db3b19608e).

## Table of Contents

- [Usage](#usage)
  - [Enabling YouTube Capabilities](#enabling-youtube-capabilities)
  - [Basic Trimming](#basic-trimming)
  - [Fit-Checking](#fit-checking)
- [Installation](#installation)
- [License](#license)

## Usage

### _**(Optional)** Enabling YouTube Capabilities_

> [!NOTE]
> This section is only needed if you're going to be passing in YouTube playlists/videos as arguments.

Steps:

- **Get your API key for the YouTube Data API (v3) from the [Google Cloud Console](https://console.cloud.google.com/)**.
- Create a new `.trimsecrc` file at your `$HOME` (on Windows: `C:\Users\<USERNAME>`) directory with this structure:
  ```toml
  api_key = "YOUR_API_KEY_HERE"
  ```
- Or, set it as `TRIMSEC_YOUTUBE_KEY` in your environment.
  - A good choice is to use [direnv](https://github.com/direnv/direnv) and create a `.envrc` file in your `$HOME`, then run `direnv allow` in your home directory from the terminal to set it as an isolated environment variable, although beware that the variable won't be available in any other directories.

### 1. Basic Trimming

#### For eyeballed durations:

To calculate saved time, you run the `trim` command as follows:

```bash
ts trim <duration> <speed>

# Example:
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

Instead of using the `trim` command, you'll be using the `yt` command:

```bash
ts yt -l https://www.youtube.com/watch?v=D4iiKkjGJmU -m 1.25x
```

You can also throw in a YouTube playlist in (almost) any format you want, and it'd show the total time saved based on the multiplier:

```bash
ts yt -l "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS" -m 1.8x
```

As you can see, the link strings are getting quite _big_. To solve this, a clipboard-fetcher comes included. Use it by just skipping the link argument:

```bash
# reads from clipboard
ts yt -m 1.8x

# disable clipboard using -n/--no-clip:
ts yt -l "https://youtube.com/..." --no-clip -m 1.8x
```

For traversing only a certain amount of items in a playlist (starting from the 1st item), use this:

```bash
ts yt --max-items 7 -l "SOME_PLAYLIST_URL" -m 1.8x
```

### Fit-Checking

This feature can be used to check whether a certain YouTube video/playlist fits in the day, or a given duration. You basically run:

```bash
ts fitcheck [OPTIONS]
```

A number of use-cases could be listed as follows:

```bash
# link grabbed from clipboard; budget is today
ts fitcheck

# same as above, but shorter
ts fc

# link pasted manually; budget is today
ts fc --link "https://youtube.com/..."

# shortened param names; budget is 2 hours and 4 minutes
ts fc -l "https://youtube.com/..." -b 2h4m

# youtube playlist + item cap
ts fc -l "https://youtube.com/playlist?..." --max-items 5

# disable clipboard functionalities intentionally
ts fc --link "https://youtube.com/..." --no-clip
```

As you can see by now, most of the parameters and flags are the same as the `trim` command, so it is worthwhile to check both documentations and compare-contrast between what to use and what to not.

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
