<div align="center">

<img src="assets/sprite.png" width="150px">

# <img src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" width="40px"> trimsec
### Strategic (& fast) content consumption planner.

![test](https://github.com/user-attachments/assets/45911e26-053e-4364-80d2-4237c2fdac15)

[![Unit Tests](https://github.com/hitblast/trimsec/actions/workflows/tests.yml/badge.svg)](https://github.com/hitblast/trimsec/actions/workflows/tests.yml)

</div>

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

As shown in the demo above, you can use `trimsec` by running the following command:

```bash
$ trimsec <duration> <speed>
```

Here, `<speed>` is the speed multiplier you are using, and `<duration>` is the
duration of the video you are watching. For example, if you are watching a 1-hour
video at 2x speed, you can run the following command:

```bash
$ trimsec 1h 2x
```

This will output the time you saved by watching the video at 2x speed. You can
also combine multiple duration indicators and float-point speed multipliers. For
example:

```bash
$ trimsec 1h30m 1.5x
```

For convenience in *some* cases, instead of using plain integers for duration,
you can also use floating-point numbers. For example:

```bash
# Equivalent to `2h 1.5x`.
$ trimsec 1.5h30m 1.5x
```

In order to calculate multiple durations at once with the same speed multiplier,
you can use the following pattern:

```bash
# Use a '+' to separate the durations.
$ trimsec 1h30m+2h50m 1.25x
```

Here we can clearly see that all of the commands above follow a similar pattern:

- While specifying the duration, you must use any combination of the following formats:
  - `<days>d` for days
  - `<hours>h` for hours
  - `<minutes>m` for minutes
  - `<seconds>s` for seconds
- The speed multiplier must be a float-point number, and *can* suffixed with `x`, such as:
  - `1.5x` indicates 1.5 times the original speed.
  - But, `1.5` is also valid and indicates the same speed multiplier.

> [!NOTE]
> For more information on the available flags and options, type `trimsec --help`.

## Installation

---

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
