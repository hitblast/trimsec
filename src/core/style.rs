use std::env;

use crate::cli::args::ColorMode;

pub struct Style {
    red: &'static str,
    reset: &'static str,
    bold: &'static str,
    green: &'static str,
}

impl Style {
    /// Determines the color palette for trimsec.
    pub fn determine(color_mode: ColorMode) -> Self {
        let defbool = env::var("NO_COLOR").ok().is_some();

        let style = Style::new(if defbool {
            false
        } else {
            match color_mode {
                ColorMode::Always => true,
                ColorMode::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
                ColorMode::Never => false,
            }
        });

        style
    }

    pub fn red(&self) -> &str {
        self.red
    }
    pub fn reset(&self) -> &str {
        self.reset
    }
    pub fn green(&self) -> &str {
        self.green
    }
    pub fn bold(&self) -> &str {
        self.bold
    }

    fn new(colors: bool) -> Self {
        if colors {
            Self {
                red: "\u{001b}[31m",
                reset: "\u{001b}[0m",
                bold: "\u{001b}[1m",
                green: "\u{001b}[32m",
            }
        } else {
            Self {
                red: "",
                reset: "",
                bold: "",
                green: "",
            }
        }
    }
}
