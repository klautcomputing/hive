use crate::bug::Bug;
use crate::color::Color;
use crate::game_error::GameError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Clone, Hash, Eq, Copy)]
pub struct Piece {
    pub bug: Bug,
    pub color: Color,
    pub order: Option<i8>,
}

impl Piece {
    pub fn new(bug: Bug, color: Color, order: Option<i8>) -> Piece {
        Piece { bug, color, order }
    }

    pub fn from_string(s: &str) -> Result<Piece, GameError> {
        let color = Color::from_str(&s.chars().next().unwrap().to_string())?;
        let bug = Bug::from_str(&s.chars().nth(1).unwrap().to_string())?;
        let mut order = None;
        if let Some(ch) = s.chars().nth(2) {
            order = Some(ch.to_string().parse().unwrap());
        }
        Ok(Piece::new(bug, color, order))
    }

    pub fn is_color(&self, color: &Color) -> bool {
        *color == self.color
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(order) = self.order {
            write!(f, "{}{}{}", self.color, self.bug, order)
        } else {
            write!(f, "{}{} ", self.color, self.bug)
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(order) = self.order {
            write!(f, "{}{}{}", self.color, self.bug, order)
        } else {
            write!(f, "{}{} ", self.color, self.bug)
        }
    }
}
