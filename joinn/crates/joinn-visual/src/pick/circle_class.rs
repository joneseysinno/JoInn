//! Classify a point against a circle.

use super::Class;
use super::squared_class::squared_class;

/// `s = |p - c|²` against `(r − EDGE)²` and `(r + EDGE)²`. `None` on overflow.
pub(crate) fn circle_class(p: (i128, i128), c: (i128, i128), r: i128) -> Option<Class> {
    let dx = p.0.checked_sub(c.0)?;
    let dy = p.1.checked_sub(c.1)?;
    let s = dx.checked_mul(dx)?.checked_add(dy.checked_mul(dy)?)?;
    squared_class(s, r)
}

#[cfg(test)]
mod tests {
    use super::circle_class;
    use crate::pick::{Class, EDGE};

    // One sixteenth of a pixel.
    const S: i128 = EDGE;

    #[test]
    fn a_pixel_exactly_on_the_edge_is_edge() {
        assert_eq!(circle_class((64 * S, 0), (0, 0), 64 * S), Some(Class::Edge));
    }

    #[test]
    fn a_pixel_one_sixteenth_inside_is_inside() {
        assert_eq!(
            circle_class((63 * S, 0), (0, 0), 64 * S),
            Some(Class::Inside)
        );
        assert_eq!(
            circle_class((65 * S, 0), (0, 0), 64 * S),
            Some(Class::Outside)
        );
    }

    #[test]
    fn a_pixel_two_sixteenths_inside_is_inside() {
        assert_eq!(
            circle_class((0, -62 * S), (0, 0), 64 * S),
            Some(Class::Inside)
        );
    }

    #[test]
    fn a_pixel_off_the_axes_between_the_bands_is_edge() {
        assert_eq!(
            circle_class((45 * S, 45 * S), (0, 0), 64 * S),
            Some(Class::Edge)
        );
    }

    #[test]
    fn a_circle_thinner_than_the_band_holds_nothing_inside() {
        assert_eq!(circle_class((0, 0), (0, 0), S / 2), Some(Class::Edge));
    }
}
