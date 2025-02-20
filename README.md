<img src="assets/sprite.png" width="200px" align="right">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec

[![Unit Tests](https://github.com/hitblast/trimsec/actions/workflows/tests.yml/badge.svg)](https://github.com/hitblast/trimsec/actions/workflows/tests.yml)
[![Release Builds](https://github.com/hitblast/trimsec/actions/workflows/release.yml/badge.svg)](https://github.com/hitblast/trimsec/actions/workflows/release.yml)

Strategic (& fast) content consumption planner.<br>

> [!NOTE]
> This project is still under active development. New ideas of improvement are always welcome!

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
- [Installation](#installation)
  - [cargo](#cargo) [rust]
  - [mise](#mise) [rust]
- [Manual installation](#manual-installation)
- [Contributors](#contributors)
- [Changelog](#changelog)
- [License](#license)

## Overview

[trimsec](https://crates.io/crates/trimsec) provides an easy way to accurately
estimate how much time you save on watching videos when using speed multipliers.
This is essentially helpful when watching recorded classes or documentaries
online, in case you do not have enough time to spare based on your situation.

Since I am also a student and both coordinating between my routine, as well as
manually doing the math required to estimate my overall hours for study wasn't
an option I'd consider - I chose to program a tool that would automate the
process for me.

If you like this tool, **consider starring the repository ✨** on GitHub! It
really encourages me to build more open-source tools like this. :D

## Usage

> [!NOTE]
> For more information on all available commands, type `trimsec --help`.

Trimsec provides two core functionalities: calculating saved time with a speed
multiplier and managing your time bank data. This guide explains how to use both
features in a seamless flow.

To calculate saved time, you run the trim command as follows:

```bash
$ trimsec trim <duration> <speed>
```

Here, `<duration>` represents the length of the video you are watching (using
any combination of `<days>d`, `<hours>h`, `<minutes>m`, and `<seconds>s`), and
`<speed>` is the speed multiplier—a floating-point number that can optionally be
suffixed with an `x` (for example, both `1.5x` and `1.5` are valid). For
instance, if you are watching a 1-hour video at 2x speed, execute:

```bash
$ trimsec trim 1h 2x
```

This command outputs the time you saved by watching the video at 2x speed. You
can also combine multiple duration indicators and apply floating-point
multipliers, such as:

```bash
$ trimsec trim 1h30m 1.5x
```

For convenience, in some cases you might use floating-point numbers for the
duration itself. For example:

```bash
# Equivalent to `2h 1.5x`.
$ trimsec trim 1.5h30m 1.5x
```

If you wish to calculate saved time for multiple durations at once using the
same speed multiplier, separate the durations with a '+' sign:

```bash
$ trimsec trim 1h30m+2h50m 1.25x
```

Each of these commands follows a consistent pattern for specifying durations and multipliers.

In addition to calculating saved time, every trim command interacts with a “time
bank” that logs the total amount of saved time per day in a JSON file. To manage
this time bank, you have several options:

- To display the current saved time for each day, use:

```bash
$ trimsec bank show
```

- To reset the time bank, run:

```bash
$ trimsec bank reset
```

- And if you want to know the absolute path to the time bank file, simply execute:

```bash
$ trimsec bank path
```

---

## Installation

### cargo

If you have [Rust](https://rust-lang.org/) and [Cargo](https://crates.io/)
installed, you can install `trimsec` by running the following command:

```bash
$ cargo install trimsec
```

### mise

To install `trimsec` as a tool using [mise](https://github.com/jdx/mise), use
the following command:

```bash
$ mise use -g cargo:trimsec
```

> [!NOTE]
> These methods are subject to change as the library is in stable development.

---

### Manual installation

Standalone binaries for this project are all available at the [GitHub
Releases](https://github.com/hitblast/trimsec/releases) page. The binaries are
produced by automated GitHub Actions workflows on three distinct runners and
should run on all their respective platforms.

In case, however, if you prefer building standalone binaries native to your
machine, here's a few commands to help you:

```bash
# Clone the repository.
$ git clone https://github.com/hitblast/trimsec.git
$ cd trimsec

# Output: target/release/trimsec
$ cargo build --release
```

Either you download the binary from the mentioned page above or manually
compile, you will have to move it to somewhere which can be directly pointed at
using the `$PATH` variable. Here's an example:

```bash
$ mv target/release/trimsec ~/.local/bin/

# (Optional) This code appends the binary path to the PATH environment variable.
$ echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
$ source ~/.bashrc

# Now you can run the binary from anywhere.
$ trimsec 1h20m 1.75x
```

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="http://furtidev.github.io"><img src="https://avatars.githubusercontent.com/u/44488750?v=4?s=100" width="100px;" alt="furtidev"/><br /><sub><b>furtidev</b></sub></a><br /><a href="#code-furtidev" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## Changelog

For release notes and changelog, please refer to the [CHANGELOG.md](https://github.com/hitblast/trimsec/blob/main/CHANGELOG.md) file.

## License

This project is licensed under the [MIT License](LICENSE).
