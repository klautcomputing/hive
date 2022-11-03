use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

// This kind of doesn't make sense but it's nice to be able to create some other data structres
// with ::default()
impl Default for Color {
    fn default() -> Self { Color::Black }
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn from_str(s: &str) -> Color {
        match s {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("That's not a color!"),
        }
    }

    pub fn to_html_color(&self) -> &str {
        match self {
            Self::Black => "grey",
            Self::White => "white",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::White => "w",
            Color::Black => "b",
        };
        write!(f, "{}", color)
    }
}