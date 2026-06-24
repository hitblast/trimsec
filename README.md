<img src="assets/sprite.png" width="200px" align="right">

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
$ ts trim <duration> <speed>
```

Here, `<duration>` represents the length of the video you are watching (using
any combination of `<days>d`, `<hours>h`, `<minutes>m`, and `<seconds>s`), and
`<speed>` is the speed multiplier; a float that can optionally be
suffixed with an `x` (for example, both `1.5x` and `1.5` are valid). For
instance, if you are watching a 1-hour video at 2x speed, execute:

```bash
$ ts trim 1h 2x
```

This command outputs the time you saved by watching the video at 2x speed. You
can also combine multiple duration indicators and apply floating-point
multipliers, such as:

```bash
$ ts trim 1h30m 1.5x
```

For convenience, in some cases you might use floating-point numbers for the
duration itself. For example:

```bash
# Equivalent to `2h 1.5x`.
$ ts trim 1.5h30m 1.5x
```

If you wish to calculate saved time for multiple durations at once using the
same speed multiplier, separate the durations with a '+' sign:

```bash
$ ts trim 1h30m+2h50m 1.25x
```

To summarize, *any combination of durations and multipliers can be used.*

### For YouTube videos:

The above applies for trimming certain YouTube videos as well, we just use the `yt` command, except we replace the `duration` field with a YouTube URL:

```bash
$ trimsec yt https://www.youtube.com/watch?v=D4iiKkjGJmU 1.25x
```

## Installation

### cargo

```bash
$ cargo install trimsec
```

### mise

```bash
$ mise use -g cargo:trimsec
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
