//! T1: where the curve starts and ends.

use joinn_frame::Verdict;

use crate::point::Point;
use crate::refuse::refuse;

/// 4ⁿ nodes; node 0 at (0, 0); the last at (2ⁿ − 1, 0).
pub(crate) fn t1_ends(order: u32, nodes: &[Point]) -> Verdict<String> {
    let count = 1usize << (2 * order);
    let side = 1u64 << order;
    let want_last = Point { x: side - 1, y: 0 };
    match (nodes.len() == count, nodes.first(), nodes.last()) {
        (true, Some(first), Some(last))
            if *first == (Point { x: 0, y: 0 }) && *last == want_last =>
        {
            Verdict::Ok(format!(
                "node 0 at (0, 0); node {} at ({}, 0)",
                count - 1,
                side - 1
            ))
        }
        _ => refuse(format!(
            "tower: order {order} has {} nodes from {:?} to {:?}; acceptance is {count} nodes from (0, 0) to ({}, 0)",
            nodes.len(),
            nodes.first().map(|p| (p.x, p.y)),
            nodes.last().map(|p| (p.x, p.y)),
            side - 1
        )),
    }
}
