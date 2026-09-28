//! Classify a point against a rounded rectangle.

use super::Class;
use super::squared_class::squared_class;

/// Centre `c`, half-extent `h`, radius `r`, all in sixteenths of a pixel.
/// `q = |p − c| − (h − r)`. `None` on overflow.
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
        return Some(if d <= -1 {
            Class::Inside
        } else if d >= 1 {
            Class::Outside
        } else {
            Class::Edge
        });
    }
    let (ex, ey) = (qx.max(0), qy.max(0));
    let s = ex.checked_mul(ex)?.checked_add(ey.checked_mul(ey)?)?;
    squared_class(s, r, 1)
}

#[cfg(test)]
mod tests {
    use super::round_rect_class;
    use crate::pick::Class;

    const C: (i128, i128) = (0, 0);
    const H: (i128, i128) = (320, 160);
    const R: i128 = 64;

    #[test]
    fn a_pixel_exactly_on_a_straight_edge_is_edge() {
        assert_eq!(round_rect_class((320, 0), C, H, R), Some(Class::Edge));
        assert_eq!(round_rect_class((0, -160), C, H, R), Some(Class::Edge));
    }

    #[test]
    fn a_pixel_one_sixteenth_inside_a_straight_edge_is_inside() {
        assert_eq!(round_rect_class((319, 0), C, H, R), Some(Class::Inside));
        assert_eq!(round_rect_class((321, 0), C, H, R), Some(Class::Outside));
    }

    #[test]
    fn a_pixel_two_sixteenths_inside_is_inside() {
        assert_eq!(round_rect_class((318, 0), C, H, R), Some(Class::Inside));
    }

    #[test]
    fn a_corner_is_judged_on_its_circle() {
        // The corner circle's centre is (h − r) = (256, 96).
        assert_eq!(
            round_rect_class((256 + 45, 96 + 45), C, H, R),
            Some(Class::Edge)
        );
        assert_eq!(
            round_rect_class((256 + 39, 96 + 52), C, H, R),
            Some(Class::Outside)
        );
        assert_eq!(
            round_rect_class((256 + 33, 96 + 33), C, H, R),
            Some(Class::Inside)
        );
    }
}
