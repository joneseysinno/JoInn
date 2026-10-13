//! A segment's direction: N, E, S or W, derived from its two nodes, never
//! stored.

mod directions;
mod name;
mod reverse;
mod step;

pub use directions::directions;

/// One unit step on the grid.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    /// `y + 1`.
    N,
    /// `x + 1`.
    E,
    /// `y − 1`.
    S,
    /// `x − 1`.
    W,
}

#[cfg(test)]
mod tests {
    use super::{Direction, directions};
    use crate::point::{Point, point};

    #[test]
    fn order_1_reads_n_e_s_and_a_jump_has_no_direction() {
        let nodes: Vec<Point> = (0..4).map(|d| point(1, d)).collect();
        assert_eq!(
            directions(&nodes),
            [Some(Direction::N), Some(Direction::E), Some(Direction::S)]
        );
        let jump = [Point { x: 0, y: 0 }, Point { x: 1, y: 1 }];
        assert_eq!(directions(&jump), [None]);
        assert_eq!(Direction::N.reverse(), Direction::S);
        assert_eq!(Direction::E.reverse().name(), "W");
    }
}
