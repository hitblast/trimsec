use crate::{
    cli::{
        args::{ColorMode, Command},
        flags::Flags,
    },
    core::style::Style,
};
use anyhow::Result;

pub mod fits;
pub mod key;
pub mod list;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self, clip: bool, color: ColorMode) -> Result<()> {
        let style = Style::determine(color);
        let flags = Flags { clip };

        match self {
            Command::Fits(fits_cmd) => fits_cmd.run(&flags, &style),
            Command::Trim(trim_cmd) => trim_cmd.run(&flags, &style),
            Command::Yt(yt_cmd) => yt_cmd.run(&flags, &style),
            Command::List(list_cmd) => list_cmd.run(&flags, &style),
            Command::Key(key_cmd) => key_cmd.run(&flags, &style),
        }
    }
}

pub trait Runnable {
    fn run(self, flags: &Flags, style: &Style) -> Result<()>;
}
