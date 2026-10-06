//! Which side of a folded node faces the knot.

/// 0 left, 1 right, 2 top, 3 bottom: left if the knot's `x` (in the node's
/// chart) is below 0; else right if it is at least the node's width; else top
/// if its `y` is below 0; else bottom.
pub fn side(origin: (i64, i64), size: (i64, i64), knot: (i64, i64)) -> usize {
    let (x, y) = (knot.0 - origin.0, knot.1 - origin.1);
    if x < 0 {
        0
    } else if x >= size.0 {
        1
    } else if y < 0 {
        2
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::side;
    use crate::charts::SYSTEM_SIZE;

    #[test]
    fn each_of_the_four_sides_by_the_knot() {
        let at = (100, 200);
        assert_eq!(side(at, SYSTEM_SIZE, (92, 276)), 0);
        assert_eq!(side(at, SYSTEM_SIZE, (404, 0)), 1);
        assert_eq!(side(at, SYSTEM_SIZE, (248, 192)), 2);
        assert_eq!(side(at, SYSTEM_SIZE, (248, 360)), 3);
        // Left wins over top at a corner; right over bottom.
        assert_eq!(side(at, SYSTEM_SIZE, (99, 199)), 0);
        assert_eq!(side(at, SYSTEM_SIZE, (404, 352)), 1);
    }
}
