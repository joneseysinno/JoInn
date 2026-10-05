//! Classify a point against a capsule (a wire or a text stroke).

use super::mul_wide::mul_wide;
use super::squared_class::squared_class;
use super::{Class, EDGE};

/// From `a` to `b` with half-width `w`, in 2^-32 px. `t = (p - a)·(b - a)`,
/// `L = |b - a|²`. Past either end the capsule is a circle; between them
/// `c = (p - a) × (b - a)` is compared as `c²` against `(w ± EDGE)²·L`, in
/// 256 bits. `None` on overflow.
pub(crate) fn capsule_class(
    p: (i128, i128),
    a: (i128, i128),
    b: (i128, i128),
    w: i128,
) -> Option<Class> {
    let (dx, dy) = (b.0.checked_sub(a.0)?, b.1.checked_sub(a.1)?);
    let (px, py) = (p.0.checked_sub(a.0)?, p.1.checked_sub(a.1)?);
    let t = px.checked_mul(dx)?.checked_add(py.checked_mul(dy)?)?;
    let l = dx.checked_mul(dx)?.checked_add(dy.checked_mul(dy)?)?;
    if t <= 0 {
        let s = px.checked_mul(px)?.checked_add(py.checked_mul(py)?)?;
        return squared_class(s, w);
    }
    if t >= l {
        let (qx, qy) = (p.0.checked_sub(b.0)?, p.1.checked_sub(b.1)?);
        let s = qx.checked_mul(qx)?.checked_add(qy.checked_mul(qy)?)?;
        return squared_class(s, w);
    }
    let c = px.checked_mul(dy)?.checked_sub(py.checked_mul(dx)?)?;
    let c2 = mul_wide(c.unsigned_abs(), c.unsigned_abs());
    let l = l.unsigned_abs();
    let lo = w.checked_sub(EDGE)?;
    let hi = w.checked_add(EDGE)?.unsigned_abs();
    let inside = lo >= 0 && {
        let lo = lo.unsigned_abs();
        c2 <= mul_wide(lo.checked_mul(lo)?, l)
    };
    let outside = c2 >= mul_wide(hi.checked_mul(hi)?, l);
    Some(if inside {
        Class::Inside
    } else if outside {
        Class::Outside
    } else {
        Class::Edge
    })
}

#[cfg(test)]
mod tests {
    use super::capsule_class;
    use crate::pick::{Class, EDGE};

    // One sixteenth of a pixel.
    const S: i128 = EDGE;
    const A: (i128, i128) = (0, 0);
    const B: (i128, i128) = (1600 * S, 0);

    #[test]
    fn a_pixel_exactly_on_a_straight_edge_is_edge() {
        assert_eq!(
            capsule_class((800 * S, 64 * S), A, B, 64 * S),
            Some(Class::Edge)
        );
    }

    #[test]
    fn a_pixel_one_sixteenth_inside_a_straight_edge_is_inside() {
        assert_eq!(
            capsule_class((800 * S, 63 * S), A, B, 64 * S),
            Some(Class::Inside)
        );
        assert_eq!(
            capsule_class((800 * S, 65 * S), A, B, 64 * S),
            Some(Class::Outside)
        );
    }

    #[test]
    fn a_pixel_two_sixteenths_inside_is_inside() {
        assert_eq!(
            capsule_class((800 * S, -62 * S), A, B, 64 * S),
            Some(Class::Inside)
        );
    }

    #[test]
    fn past_an_end_the_capsule_is_a_circle() {
        assert_eq!(capsule_class((-64 * S, 0), A, B, 64 * S), Some(Class::Edge));
        assert_eq!(
            capsule_class((1663 * S, 0), A, B, 64 * S),
            Some(Class::Inside)
        );
        assert_eq!(
            capsule_class((1665 * S, 0), A, B, 64 * S),
            Some(Class::Outside)
        );
    }

    #[test]
    fn a_diagonal_wire_is_judged_exactly() {
        // (0,0) → (300,400); the unit normal is (4,-3)/5. From the midpoint
        // (150,200): 13 normals is 65 away, 12 is 60, and (201,162) is 63.6.
        let a = (0, 0);
        let b = (300 * S, 400 * S);
        let w = 64 * S;
        assert_eq!(
            capsule_class((202 * S, 161 * S), a, b, w),
            Some(Class::Outside)
        );
        assert_eq!(
            capsule_class((201 * S, 162 * S), a, b, w),
            Some(Class::Edge)
        );
        assert_eq!(
            capsule_class((198 * S, 164 * S), a, b, w),
            Some(Class::Inside)
        );
    }

    #[test]
    fn a_long_wire_far_from_the_origin_compares_past_128_bits() {
        // 2^20 px long, 2^21 px from the origin: c² is near 2^172.
        let a = (1 << 53, 1 << 53);
        let b = (a.0 + (1 << 52), a.1);
        let w = 64 * S;
        assert_eq!(
            capsule_class((a.0 + (1 << 51), a.1 + 63 * S), a, b, w),
            Some(Class::Inside)
        );
        assert_eq!(
            capsule_class((a.0 + (1 << 51), a.1 + 65 * S), a, b, w),
            Some(Class::Outside)
        );
    }
}
