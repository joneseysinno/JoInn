//! Nodes of the curve: at order n, 4ⁿ nodes on the integer grid of side 2ⁿ,
//! node d at `point(n, d)`, the standard Hilbert map. Orders up to 31.

mod hilbert;
mod nodes;

pub use hilbert::point;
pub use nodes::nodes;

/// An integer grid point; `y` grows north.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point {
    /// East.
    pub x: u64,
    /// North.
    pub y: u64,
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::{Point, nodes, point};
    use crate::dim::Dim;

    #[test]
    fn order_1_is_a_cup_open_to_the_south() {
        let got: Vec<(u64, u64)> = (0..4).map(|d| point(1, d)).map(|p| (p.x, p.y)).collect();
        assert_eq!(got, [(0, 0), (0, 1), (1, 1), (1, 0)]);
    }

    #[test]
    fn every_node_of_an_order_is_a_distinct_grid_point() {
        for order in 0..=6 {
            let Verdict::Ok(all) = nodes(Dim(2), order) else {
                panic!("order {order}");
            };
            let side = 1u64 << order;
            assert_eq!(all.len() as u64, side * side);
            let mut sorted = all.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), all.len(), "order {order}");
            assert!(all.iter().all(|p| p.x < side && p.y < side));
        }
        assert_eq!(point(0, 0), Point { x: 0, y: 0 });
        assert!(nodes(Dim(3), 1).is_refused());
    }
}
