//! Classify a point against a rounded rectangle.

use super::squared_class::squared_class;
use super::{Class, EDGE};

/// Centre `c`, half-extent `h`, radius `r`, all in 2^-32 px.
/// `q = |p - c| - (h - r)`. `None` on overflow.
pub(crate) fn round_rect_class(
    p: (i128, i128),
    c: (i128, i128),
    h: (i128, i128),
    r: i128,
) -> Option<Class> {
    let qx =
        p.0.checked_sub(c.0)?
            .checked_abs()?
            .checked_sub(h.0.checked_sub(r)?)?;
    let qy =
        p.1.checked_sub(c.1)?
            .checked_abs()?
            .checked_sub(h.1.checked_sub(r)?)?;
    if qx <= 0 && qy <= 0 {
        let d = qx.max(qy).checked_sub(r)?;
        return Some(if d <= -EDGE {
            Class::Inside
        } else if d >= EDGE {
            Class::Outside
        } else {
            Class::Edge
        });
    }
    let (ex, ey) = (qx.max(0), qy.max(0));
    let s = ex.checked_mul(ex)?.checked_add(ey.checked_mul(ey)?)?;
    squared_class(s, r)
}

#[cfg(test)]
mod tests {
    use super::round_rect_class;
    use crate::pick::{Class, EDGE};

    // One sixteenth of a pixel.
    const S: i128 = EDGE;
    const C: (i128, i128) = (0, 0);
    const H: (i128, i128) = (320 * S, 160 * S);
    const R: i128 = 64 * S;

    #[test]
    fn a_pixel_exactly_on_a_straight_edge_is_edge() {
        assert_eq!(round_rect_class((320 * S, 0), C, H, R), Some(Class::Edge));
        assert_eq!(round_rect_class((0, -160 * S), C, H, R), Some(Class::Edge));
    }

    #[test]
    fn a_pixel_one_sixteenth_inside_a_straight_edge_is_inside() {
        assert_eq!(round_rect_class((319 * S, 0), C, H, R), Some(Class::Inside));
        assert_eq!(
            round_rect_class((321 * S, 0), C, H, R),
            Some(Class::Outside)
        );
    }

    #[test]
    fn a_pixel_two_sixteenths_inside_is_inside() {
        assert_eq!(round_rect_class((318 * S, 0), C, H, R), Some(Class::Inside));
    }

    #[test]
    fn a_corner_is_judged_on_its_circle() {
        // The corner circle's centre is (h - r) = (256, 96) sixteenths.
        assert_eq!(
            round_rect_class(((256 + 45) * S, (96 + 45) * S), C, H, R),
            Some(Class::Edge)
        );
        assert_eq!(
            round_rect_class(((256 + 39) * S, (96 + 52) * S), C, H, R),
            Some(Class::Outside)
        );
        assert_eq!(
            round_rect_class(((256 + 33) * S, (96 + 33) * S), C, H, R),
            Some(Class::Inside)
        );
    }
}
