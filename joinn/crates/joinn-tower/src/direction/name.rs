//! A direction's printed name.

use super::Direction;

impl Direction {
    /// `N`, `E`, `S` or `W`.
    pub fn name(self) -> &'static str {
        match self {
            Direction::N => "N",
            Direction::E => "E",
            Direction::S => "S",
            Direction::W => "W",
        }
    }
}
