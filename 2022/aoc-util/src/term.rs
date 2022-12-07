/// Attributes text on a XTerm can be styled with
pub struct TermAttr {
    pub bold: bool,
    pub underlined: bool,
    pub color: Option<TermColor>,
}

/// The XTerm colors
pub enum TermColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl TermColor {
    pub fn as_xterm_attr(&self) -> &'static str {
        use TermColor::*;
        match self {
            Black => "30",
            Red => "31",
            Green => "32",
            Yellow => "33",
            Blue => "34",
            Magenta => "35",
            Cyan => "36",
            White => "37",
            BrightBlack => "90",
            BrightRed => "91",
            BrightGreen => "92",
            BrightYellow => "93",
            BrightBlue => "94",
            BrightMagenta => "95",
            BrightCyan => "96",
            BrightWhite => "97",
        }
    }
}
