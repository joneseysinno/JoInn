//! T2: consecutive nodes are one unit step apart.

use joinn_frame::Verdict;

use crate::direction::directions;
use crate::point::Point;
use crate::refuse::refuse;

/// Every segment is one unit step.
pub(crate) fn t2_steps(order: u32, nodes: &[Point]) -> Verdict<String> {
    let dirs = directions(nodes);
    if let Some(i) = dirs.iter().position(Option::is_none) {
        let (a, b) = (nodes[i], nodes[i + 1]);
        return refuse(format!(
            "tower: order {order} node {i} at ({}, {}) to node {} at ({}, {}) is not one unit step; acceptance is consecutive nodes one step apart",
            a.x,
            a.y,
            i + 1,
            b.x,
            b.y
        ));
    }
    Verdict::Ok(format!("{} steps, each one unit", dirs.len()))
}
