//! T6: the curve is its own mirror.

use joinn_frame::Verdict;

use crate::direction::{Direction, directions};
use crate::point::Point;
use crate::refuse::refuse;

/// Reverse the direction list, reverse each direction, swap E↔W: the list is
/// unchanged.
pub(crate) fn t6_mirror(order: u32, nodes: &[Point]) -> Verdict<String> {
    let dirs = directions(nodes);
    let mirror = |d: Direction| match d.reverse() {
        Direction::E => Direction::W,
        Direction::W => Direction::E,
        other => other,
    };
    let name = |d: Option<Direction>| d.map_or("no step", Direction::name);
    let n = dirs.len();
    for (i, d) in dirs.iter().enumerate() {
        let m = n - 1 - i;
        let got = dirs[m].map(mirror);
        if got != *d {
            return refuse(format!(
                "tower: order {order} segment {i} runs {}, but segment {m} mirrored runs {}; acceptance is the mirror unchanged",
                name(*d),
                name(got)
            ));
        }
    }
    Verdict::Ok(format!("mirror holds over {n} directions"))
}
