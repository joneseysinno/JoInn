//! The direction from one point to the next.

use super::Direction;
use crate::point::Point;

impl Direction {
    /// `Some` only when `b` is one unit step from `a`.
    pub fn step(a: Point, b: Point) -> Option<Direction> {
        if a.x == b.x && b.y.checked_sub(a.y) == Some(1) {
            Some(Direction::N)
        } else if a.x == b.x && a.y.checked_sub(b.y) == Some(1) {
            Some(Direction::S)
        } else if a.y == b.y && b.x.checked_sub(a.x) == Some(1) {
            Some(Direction::E)
        } else if a.y == b.y && a.x.checked_sub(b.x) == Some(1) {
            Some(Direction::W)
        } else {
            None
        }
    }
}
