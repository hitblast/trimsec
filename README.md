<img src="assets/trimsec.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

> [!NOTE]
> Want a very simple, basic version of it? Try [trimsec.c](https://gist.github.com/hitblast/3898c05bc13385507a0c37db3b19608e).

## Table of Contents

- [Usage](#usage)
- [Installation](#installation)
- [License](#license)

## Usage

### For basic trimming:

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

### For YouTube videos:

> [!NOTE]
> **Get your API key for the YouTube Data API (v3) from the [Google Cloud Console](https://console.cloud.google.com/)** in order to make this feature work. You must set it as `TRIMSEC_YOUTUBE_KEY` in your environment.

The above applies for trimming certain YouTube videos as well, we just use the `yt` command, except we replace the `duration` field with a YouTube URL:

```bash
ts yt -l https://www.youtube.com/watch?v=D4iiKkjGJmU -m 1.25x
```

You can also throw in a YouTube playlist in (almost) any format you want, and it'd show the total time saved based on the multiplier:

```bash
ts yt -l "https://www.youtube.com/watch?v=rdXw7Ps9vxc&list=PLHXZ9OQGMqxersk8fUxiUMSIx0DBqsKZS" -m 1.8x
```

As you can see, the link strings are getting quite *big*. To solve this, a clipboard-fetcher comes included. Use it by just skipping the link argument:

```bash
# reads from clipboard as fallback
ts yt -m 1.8x

# disable clipboard fallback using -n/--no-clip
ts yt -l "https://youtube.com/..." --noclip -m 1.8x
```

For traversing only a certain amount of items in a playlist (starting from the 1st item), use this:

```bash
ts yt --max-items 7 -l "SOME_PLAYLIST_URL" -m 1.8x
```

## Installation

### Homebrew

```bash
brew tap hitblast/tap
brew trust hitblast/tap
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
