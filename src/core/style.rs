pub struct Style {
    pub red: &'static str,
    pub reset: &'static str,
    pub bold: &'static str,
    pub green: &'static str,
}

impl Style {
    pub fn new(colors: bool) -> Self {
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
