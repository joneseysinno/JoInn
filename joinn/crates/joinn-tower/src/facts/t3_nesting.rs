//! T3: nesting.

use joinn_frame::Verdict;

use crate::point::Point;
use crate::refuse::refuse;

/// `point(n, d) >> 1 == point(n − 1, d >> 2)` for every d.
pub(crate) fn t3_nesting(order: u32, nodes: &[Point], below: &[Point]) -> Verdict<String> {
    for (d, p) in nodes.iter().enumerate() {
        let half = Point {
            x: p.x >> 1,
            y: p.y >> 1,
        };
        let parent = below.get(d >> 2);
        if parent != Some(&half) {
            return refuse(format!(
                "tower: order {order} node {d} at ({}, {}) halves to ({}, {}), but order {} node {} is at {:?}; acceptance is point(n, d) >> 1 == point(n - 1, d >> 2)",
                p.x,
                p.y,
                half.x,
                half.y,
                order.saturating_sub(1),
                d >> 2,
                parent.map(|q| (q.x, q.y))
            ));
        }
    }
    Verdict::Ok(format!(
        "{} nodes nest in order {}",
        nodes.len(),
        order.saturating_sub(1)
    ))
}
