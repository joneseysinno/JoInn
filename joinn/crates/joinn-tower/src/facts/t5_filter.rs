//! T5: the level filter gives the order below.

use joinn_frame::Verdict;

use crate::direction::{Direction, directions};
use crate::level::level_filter;
use crate::point::Point;
use crate::refuse::refuse;

/// The directions of the segments with level ≥ `floor`, in order, are exactly
/// order n − 1's directions. T5 is `floor` 1.
pub fn t5_filter(order: u32, nodes: &[Point], below: &[Point], floor: u32) -> Verdict<String> {
    let kept = level_filter(&directions(nodes), floor);
    let want = directions(below);
    let name = |d: Option<&Option<Direction>>| match d {
        Some(Some(d)) => d.name(),
        Some(None) => "no step",
        None => "nothing",
    };
    for k in 0..kept.len().max(want.len()) {
        if kept.get(k) != want.get(k) {
            return refuse(format!(
                "tower: order {order} kept segment {k} (level ≥ {floor}) runs {}; order {} segment {k} runs {}; acceptance is the level filter giving order {}'s directions",
                name(kept.get(k)),
                order.saturating_sub(1),
                name(want.get(k)),
                order.saturating_sub(1)
            ));
        }
    }
    Verdict::Ok(format!(
        "{} segments of level ≥ {floor} give order {}'s {} directions",
        kept.len(),
        order.saturating_sub(1),
        want.len()
    ))
}
