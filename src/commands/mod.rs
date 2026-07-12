use std::env;

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

        let defbool = env::var("NO_COLOR").ok().is_some();

        let style = Style::new(if defbool {
            false
        } else {
            match color {
                ColorMode::Always => true,
                ColorMode::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
                ColorMode::Never => false,
            }
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
