use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy,)]
pub(crate) enum Color {
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
    TrueColor {
        r: u8,
        g: u8,
        b: u8,
    },
}

#[derive(Serialize, Deserialize, Clone,)]
pub(crate) struct Priority {
    pub(crate) name:  String,
    pub(crate) id:    String,
    pub(crate) color: Color,
}

#[derive(Serialize, Deserialize, Clone,)]
pub(crate) struct Author {
    pub(crate) username: String,
    pub(crate) email:    String,
    pub(crate) name:     String,
}

#[derive(Serialize, Deserialize, Clone,)]
pub(crate) struct Reminder {
    pub(crate) priority: String,
    pub(crate) author:   String,
    pub(crate) text:     String,
}

pub(crate) struct LocalReminder {
    pub(crate) reminder: Reminder,
    pub(crate) path:     PathBuf,
}

impl Reminder {
    pub(crate) fn format(show_id: bool,) -> String {
        "".to_string()
    }
}

impl Priority {
    pub(crate) fn new(name: String, id: String, color: Color,) -> Self {
        Priority {
            name,
            id,
            color,
        }
    }
}

impl From<colored::Color,> for Color {
    fn from(arg: colored::Color,) -> Self {
        match arg {
            | colored::Color::Black => Color::Black,
            | colored::Color::Red => Color::Red,
            | colored::Color::Green => Color::Green,
            | colored::Color::Yellow => Color::Yellow,
            | colored::Color::Blue => Color::Blue,
            | colored::Color::Magenta => Color::Magenta,
            | colored::Color::Cyan => Color::Cyan,
            | colored::Color::White => Color::White,
            | colored::Color::BrightBlack => Color::BrightBlack,
            | colored::Color::BrightRed => Color::BrightRed,
            | colored::Color::BrightGreen => Color::BrightGreen,
            | colored::Color::BrightYellow => Color::BrightYellow,
            | colored::Color::BrightBlue => Color::BrightBlue,
            | colored::Color::BrightMagenta => Color::BrightMagenta,
            | colored::Color::BrightCyan => Color::BrightCyan,
            | colored::Color::BrightWhite => Color::BrightWhite,
            | colored::Color::TrueColor {
                r,
                g,
                b,
            } => Color::TrueColor {
                r,
                g,
                b,
            },
        }
    }
}

impl Into<colored::Color,> for Color {
    fn into(self,) -> colored::Color {
        match self {
            | Color::Black => colored::Color::Black,
            | Color::Red => colored::Color::Red,
            | Color::Green => colored::Color::Green,
            | Color::Yellow => colored::Color::Yellow,
            | Color::Blue => colored::Color::Blue,
            | Color::Magenta => colored::Color::Magenta,
            | Color::Cyan => colored::Color::Cyan,
            | Color::White => colored::Color::White,
            | Color::BrightBlack => colored::Color::BrightBlack,
            | Color::BrightRed => colored::Color::BrightRed,
            | Color::BrightGreen => colored::Color::BrightGreen,
            | Color::BrightYellow => colored::Color::BrightYellow,
            | Color::BrightBlue => colored::Color::BrightBlue,
            | Color::BrightMagenta => colored::Color::BrightMagenta,
            | Color::BrightCyan => colored::Color::BrightCyan,
            | Color::BrightWhite => colored::Color::BrightWhite,
            | Color::TrueColor {
                r,
                g,
                b,
            } => colored::Color::TrueColor {
                r,
                g,
                b,
            },
        }
    }
}
