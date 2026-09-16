use crate::{
    cli::args::{ColorMode, Command},
    core::style::Style,
};
use anyhow::Result;

pub mod fit;
pub mod key_set;
pub mod key_show;
pub mod list;
pub mod path;
pub mod trim;
pub mod yt;

impl Command {
    pub fn run(self, color: ColorMode) -> Result<()> {
        let style = Style::determine(color);

        match self {
            Command::Fit(fits_cmd) => fits_cmd.run(&style),
            Command::Trim(trim_cmd) => trim_cmd.run(&style),
            Command::Yt(yt_cmd) => yt_cmd.run(&style),
            Command::List(list_cmd) => list_cmd.run(&style),
            Command::Key { command } => match command {
                crate::cli::args::KeySubcmd::Show(key_show_cmd) => key_show_cmd.run(&style),
                crate::cli::args::KeySubcmd::Set(key_set_cmd) => key_set_cmd.run(&style),
            },
            Command::Path(path_cmd) => path_cmd.run(&style),
        }
    }
}

pub trait Runnable {
    fn run(self, style: &Style) -> Result<()>;
}
