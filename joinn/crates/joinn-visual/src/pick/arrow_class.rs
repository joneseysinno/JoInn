//! Classify a point against an arrowhead (a link's triangle).

use super::mul_wide::mul_wide;
use super::{Class, EDGE};

/// From base centre `a` to tip `b`, half-width `w` at the base, in 2^-32 px.
/// An arrowhead lies along a gutter, so `b − a` is axis-aligned and the
/// point's offsets along it, `s`, and across it, `n`, are exact. The base is
/// `s = 0`, the sides `D = n·len + w·s − w·len = 0` at distance
/// `D / √(len² + w²)`, compared as `D²` against `EDGE²·(len² + w²)` in 256
/// bits. `None` on overflow or an arrowhead that isn't axis-aligned.
pub(crate) fn arrow_class(
    p: (i128, i128),
    a: (i128, i128),
    b: (i128, i128),
    w: i128,
) -> Option<Class> {
    let (dx, dy) = (b.0.checked_sub(a.0)?, b.1.checked_sub(a.1)?);
    if (dx == 0) == (dy == 0) {
        return None;
    }
    let (ex, ey) = (dx.signum(), dy.signum());
    let len = dx.checked_abs()?.checked_add(dy.checked_abs()?)?;
    let (px, py) = (p.0.checked_sub(a.0)?, p.1.checked_sub(a.1)?);
    let s = px.checked_mul(ex)?.checked_add(py.checked_mul(ey)?)?;
    let n = px
        .checked_mul(ey)?
        .checked_sub(py.checked_mul(ex)?)?
        .checked_abs()?;
    let d = n
        .checked_mul(len)?
        .checked_add(w.checked_mul(s.checked_sub(len)?)?)?;
    let d2 = mul_wide(d.unsigned_abs(), d.unsigned_abs());
    let hyp = len.checked_mul(len)?.checked_add(w.checked_mul(w)?)?;
    let band = mul_wide(
        EDGE.unsigned_abs().checked_mul(EDGE.unsigned_abs())?,
        hyp.unsigned_abs(),
    );
    let side_out = d > 0 && d2 >= band;
    let side_in = d < 0 && d2 >= band;
    Some(if s <= -EDGE || s.checked_sub(len)? >= EDGE || side_out {
        Class::Outside
    } else if s >= EDGE && side_in {
        Class::Inside
    } else {
        Class::Edge
    })
}

#[cfg(test)]
mod tests {
    use super::arrow_class;
    use crate::pick::{Class, EDGE};

    // One sixteenth of a pixel. A 3-4-5 arrowhead: base half-width 960, length
    // 1280, each side 1600 long, so a point x below a side at mid-length lies
    // 0.8·x from it.
    const S: i128 = EDGE;
    const A: (i128, i128) = (0, 0);
    const B: (i128, i128) = (1280 * S, 0);
    const W: i128 = 960 * S;

    #[test]
    fn a_side_is_judged_exactly_at_mid_length() {
        let at = |n: i128| arrow_class((640 * S, n * S), A, B, W);
        assert_eq!(at(478), Some(Class::Inside));
        assert_eq!(at(479), Some(Class::Edge));
        assert_eq!(at(480), Some(Class::Edge));
        assert_eq!(at(481), Some(Class::Edge));
        assert_eq!(at(482), Some(Class::Outside));
        assert_eq!(at(-478), Some(Class::Inside));
        assert_eq!(at(-482), Some(Class::Outside));
    }

    #[test]
    fn the_base_and_the_tip_have_edge_bands() {
        let at = |s: i128| arrow_class((s * S, 0), A, B, W);
        assert_eq!(at(-1), Some(Class::Outside));
        assert_eq!(at(0), Some(Class::Edge));
        assert_eq!(at(1), Some(Class::Inside));
        assert_eq!(at(1279), Some(Class::Edge));
        assert_eq!(at(1281), Some(Class::Outside));
    }

    #[test]
    fn an_arrowhead_pointing_up_is_the_same_triangle_turned() {
        let b = (0, -1280 * S);
        assert_eq!(
            arrow_class((478 * S, -640 * S), A, b, W),
            Some(Class::Inside)
        );
        assert_eq!(
            arrow_class((-482 * S, -640 * S), A, b, W),
            Some(Class::Outside)
        );
        assert_eq!(arrow_class((0, S), A, b, W), Some(Class::Outside));
    }

    #[test]
    fn a_diagonal_or_empty_arrowhead_is_refused() {
        assert_eq!(arrow_class((0, 0), A, (S, S), W), None);
        assert_eq!(arrow_class((0, 0), A, A, W), None);
    }
}
