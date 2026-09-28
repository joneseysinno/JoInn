//! Classify a point against a capsule (a wire).

use super::Class;
use super::squared_class::squared_class;

/// From `a` to `b` with half-width `w`. `t = (p − a)·(b − a)`, `L = |b − a|²`.
/// Past either end the capsule is a circle; between them `c = (p − a) × (b − a)`
/// is compared as `c²` against `(w ∓ 1)²·L`. `None` on overflow.
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
        return squared_class(s, w, 1);
    }
    if t >= l {
        let (qx, qy) = (p.0.checked_sub(b.0)?, p.1.checked_sub(b.1)?);
        let s = qx.checked_mul(qx)?.checked_add(qy.checked_mul(qy)?)?;
        return squared_class(s, w, 1);
    }
    let c = px.checked_mul(dy)?.checked_sub(py.checked_mul(dx)?)?;
    squared_class(c.checked_mul(c)?, w, l)
}

#[cfg(test)]
mod tests {
    use super::capsule_class;
    use crate::pick::Class;

    const A: (i128, i128) = (0, 0);
    const B: (i128, i128) = (1600, 0);

    #[test]
    fn a_pixel_exactly_on_a_straight_edge_is_edge() {
        assert_eq!(capsule_class((800, 64), A, B, 64), Some(Class::Edge));
    }

    #[test]
    fn a_pixel_one_sixteenth_inside_a_straight_edge_is_inside() {
        assert_eq!(capsule_class((800, 63), A, B, 64), Some(Class::Inside));
        assert_eq!(capsule_class((800, 65), A, B, 64), Some(Class::Outside));
    }

    #[test]
    fn a_pixel_two_sixteenths_inside_is_inside() {
        assert_eq!(capsule_class((800, -62), A, B, 64), Some(Class::Inside));
    }

    #[test]
    fn past_an_end_the_capsule_is_a_circle() {
        assert_eq!(capsule_class((-64, 0), A, B, 64), Some(Class::Edge));
        assert_eq!(capsule_class((1663, 0), A, B, 64), Some(Class::Inside));
        assert_eq!(capsule_class((1665, 0), A, B, 64), Some(Class::Outside));
    }

    #[test]
    fn a_diagonal_wire_is_judged_exactly() {
        // (0,0) → (300,400); the unit normal is (4,−3)/5. From the midpoint
        // (150,200): 13 normals is 65 away, 12 is 60, and (201,162) is 63.6.
        let a = (0, 0);
        let b = (300, 400);
        assert_eq!(capsule_class((202, 161), a, b, 64), Some(Class::Outside));
        assert_eq!(capsule_class((201, 162), a, b, 64), Some(Class::Edge));
        assert_eq!(capsule_class((198, 164), a, b, 64), Some(Class::Inside));
    }
}
