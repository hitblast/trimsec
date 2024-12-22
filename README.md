<div align="center">

<img src="assets/sprite.png" width="100px">

# trimsec
### Calculate saved time on using media speed multipliers, with speed.

https://github.com/user-attachments/assets/9478947b-b768-43e3-bc99-016eb5afbfd6

</div>

## Overview

[trimsec]() provides an easy way to accurately estimate how much time you save
on watching videos when using speed multipliers. This is essentially helpful
when watching recorded classes or documentaries online, in case you do not have
enough time to spare based on your situation.

Since I am also a student and both coordinating between my routine, as well as
manually doing the math required to estimate my overall hours for study wasn't an
option I'd consider - I chose to program a tool that would automate the process for me.

## Usage

As shown in the demo above, you can use `trimsec` by running the following command:

```bash
$ trimsec <speed> <duration>
```

Here, `<speed>` is the speed multiplier you are using, and `<duration>` is the
duration of the video you are watching. For example, if you are watching a 1-hour
video at 2x speed, you can run the following command:

```bash
$ trimsec 1h 2x
```

This will output the time you saved by watching the video at 2x speed. You can also
combine multiple duration indicators and float-point speed multipliers. For example,

```bash
$ trimsec 1h30m 1.5x
```

> [!NOTE]
> Please read [Caveats](#Caveats) for more information on the supported formats.

## Installation

If you have [Rust](https://rust-lang.org/) and [Cargo](https://crates.io/)
installed, you can install `trimsec` by running the following command:

```bash
$ cargo install trimsec
```

## Manual Installation

If you do not have Rust and Cargo installed, you can clone this repository and
build the binary manually:

```bash
# clone the repository
$ git clone https://github.com/hitblast/trimsec.git
$ cd trimsec

# output: target/release/trimsec
$ cargo build --release
```

After building the binary, move the binary to a directory which is registered in
the `$PATH` environment variable. Here's an example with the `~/.local/bin/`
directory on macOS:

```bash
$ mv target/release/trimsec ~/.local/bin/

# (optional) this code appends the binary path to the PATH environment variable
$ echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
$ source ~/.bashrc

# now you can run the binary from anywhere
$ trimsec 1h20m 1.75x
```

## Caveats

- While specifying the duration, you must use any combination of the following formats:
  - `<days>d` for days
  - `<hours>h` for hours
  - `<minutes>m` for minutes
  - `<seconds>s` for seconds
- The speed multiplier must be a float-point number, and *can* suffixed with `x`, such as:
  - `1.5x` indicates 1.5 times the original speed
  - But, `1.5` is also valid and indicates the same speed multiplier

## License

This project is licensed under the [MIT License](LICENSE).
