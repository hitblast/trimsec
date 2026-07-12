use crate::{
    cli::{
        args::{ColorMode, Command},
        flags::Flags,
    },
    core::style::Style,
};
use anyhow::Result;

pub mod fitcheck;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self, no_clip: bool, color: ColorMode) -> Result<()> {
        let flags = Flags { no_clip };
        let style = Style::new(match color {
            ColorMode::Always => true,
            ColorMode::Auto => false, // TODO: work on this
            ColorMode::Never => false,
        });

        match self {
            Command::Fitcheck(fitcheck_cmd) => fitcheck_cmd.run(&flags, &style),
            Command::Trim(trim_cmd) => trim_cmd.run(&flags, &style),
            Command::Yt(yt_cmd) => yt_cmd.run(&flags, &style),
        }
    }
}

pub trait Runnable {
    fn run(self, flags: &Flags, style: &Style) -> Result<()>;
}
