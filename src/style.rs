use std::env;

use crate::args::ColorMode;
use constcat::concat;

#[derive(Default)]
pub struct Style {
    red: &'static str,
    boldred: &'static str,
    reset: &'static str,
    bold: &'static str,
    green: &'static str,
    boldgreen: &'static str,
    grey: &'static str,
    boldgrey: &'static str,
}

macro_rules! getter {
    ($x:ident) => {
        #[must_use]
        pub fn $x(&self) -> &str {
            self.$x
        }
    };
}

impl Style {
    #[must_use]
    pub fn new(color_mode: &ColorMode) -> Self {
        let colors = if env::var("NO_COLOR").ok().is_some() {
            false
        } else {
            match color_mode {
                ColorMode::Always => true,
                ColorMode::Auto => supports_color::on(supports_color::Stream::Stdout).is_some(),
                ColorMode::Never => false,
            }
        };
        if colors {
            Self::full()
        } else {
            Self::default()
        }
    }

    fn full() -> Self {
        const RED: &str = "\u{001b}[31m";
        const BOLD: &str = "\u{001b}[1m";
        const BOLDRED: &str = concat!(RED, BOLD);
        const GREEN: &str = "\u{001b}[32m";
        const BOLDGREEN: &str = concat!(GREEN, BOLD);
        const GREY: &str = "\u{001b}[90m";
        const BOLDGREY: &str = concat!(GREY, BOLD);
        const RESET: &str = "\u{001b}[0m";

        Self {
            red: RED,
            boldred: BOLDRED,
            reset: RESET,
            bold: BOLD,
            green: GREEN,
            boldgreen: BOLDGREEN,
            grey: GREY,
            boldgrey: BOLDGREY,
        }
    }

    getter!(bold);
    getter!(reset);

    getter!(red);
    getter!(boldred);
    getter!(green);
    getter!(boldgreen);
    getter!(grey);
    getter!(boldgrey);
}
