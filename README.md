<img src="assets/sprite.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

[![Unit Tests](https://github.com/hitblast/trimsec/actions/workflows/tests.yml/badge.svg)](https://github.com/hitblast/trimsec/actions/workflows/tests.yml)
[![Release Builds](https://github.com/hitblast/trimsec/actions/workflows/release.yml/badge.svg)](https://github.com/hitblast/trimsec/actions/workflows/release.yml)

Calculate time saved on using speed multipliers.

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
- [Installation](#installation)
  - [cargo](#cargo)
  - [mise](#mise)
- [Manual Installation](#manual-installation)
- [License](#license)

## Overview

I wanted a simple solution to calculate how much time I could save by using speed multipliers during lectures.
But, I did not want to hop onto the browser or even open up the calculator, or even do mind-maths for this.
Thus, I tried making a simple solution which lives in the terminal.

## Usage

To calculate saved time, you run the trim command as follows:

```bash
$ ts <duration> <speed>
```

Here, `<duration>` represents the length of the video you are watching (using
any combination of `<days>d`, `<hours>h`, `<minutes>m`, and `<seconds>s`), and
`<speed>` is the speed multiplier; a float that can optionally be
suffixed with an `x` (for example, both `1.5x` and `1.5` are valid). For
instance, if you are watching a 1-hour video at 2x speed, execute:

```bash
$ ts 1h 2x
```

This command outputs the time you saved by watching the video at 2x speed. You
can also combine multiple duration indicators and apply floating-point
multipliers, such as:

```bash
$ ts 1h30m 1.5x
```

For convenience, in some cases you might use floating-point numbers for the
duration itself. For example:

```bash
# Equivalent to `2h 1.5x`.
$ ts 1.5h30m 1.5x
```

If you wish to calculate saved time for multiple durations at once using the
same speed multiplier, separate the durations with a '+' sign:

```bash
$ ts 1h30m+2h50m 1.25x
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

## Manual Installation

Standalone binaries for this project are all available at the [GitHub
Releases](https://github.com/hitblast/trimsec/releases) page. The binaries are
produced by automated GitHub Actions workflows on three distinct runners and
should run on all their respective platforms.

For manual building, proceed with the following:

```bash
# Clone the repository.
$ git clone https://github.com/hitblast/trimsec.git
$ cd trimsec

# Output: target/release/trimsec
$ cargo build --release
```

## License

This project is licensed under the [MIT License](LICENSE).
