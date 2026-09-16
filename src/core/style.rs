use std::env;

use crate::cli::args::ColorMode;

pub struct Style {
    red: &'static str,
    boldred: String,
    reset: &'static str,
    bold: &'static str,
    green: &'static str,
    boldgreen: String,
}

impl Style {
    /// Determines the color palette for trimsec.
    #[must_use]
    pub fn determine(color_mode: ColorMode) -> Self {
        let defbool = env::var("NO_COLOR").ok().is_some();

        Style::new(if defbool {
            false
        } else {
            match color_mode {
                ColorMode::Always => true,
                ColorMode::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
                ColorMode::Never => false,
            }
        })
    }

    #[must_use]
    pub fn red(&self) -> &str {
        self.red
    }
    #[must_use]
    pub fn boldred(&self) -> &str {
        &self.boldred
    }
    #[must_use]
    pub fn reset(&self) -> &str {
        self.reset
    }
    #[must_use]
    pub fn green(&self) -> &str {
        self.green
    }
    #[must_use]
    pub fn boldgreen(&self) -> &str {
        &self.boldgreen
    }
    #[must_use]
    pub fn bold(&self) -> &str {
        self.bold
    }

    fn new(colors: bool) -> Self {
        let red = "\u{001b}[31m";
        let bold = "\u{001b}[1m";
        let boldred = red.to_owned() + bold;
        let green = "\u{001b}[32m";
        let boldgreen = green.to_owned() + bold;
        let reset = "\u{001b}[0m";

        if colors {
            Self {
                red,
                boldred,
                reset,
                bold,
                green,
                boldgreen,
            }
        } else {
            Self {
                red: "",
                boldred: "".to_string(),
                reset: "",
                bold: "",
                green: "",
                boldgreen: "".to_string(),
            }
        }
    }
}
