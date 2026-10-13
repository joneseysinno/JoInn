//! The opposite direction.

use super::Direction;

impl Direction {
    /// N↔S, E↔W.
    pub fn reverse(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}
