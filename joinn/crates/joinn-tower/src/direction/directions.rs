//! The directions of a node list's segments.

use super::Direction;
use crate::point::Point;

/// Segment i runs from node i to node i + 1; `None` where that is not one
/// unit step.
pub fn directions(nodes: &[Point]) -> Vec<Option<Direction>> {
    nodes
        .windows(2)
        .map(|w| match w {
            [a, b] => Direction::step(*a, *b),
            _ => None,
        })
        .collect()
}
