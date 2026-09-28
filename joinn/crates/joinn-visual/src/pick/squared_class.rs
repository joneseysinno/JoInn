//! Classify a squared distance against a radius, without a square root.

use super::Class;

/// `s` against `(r − 1)²·scale` and `(r + 1)²·scale`. `None` on overflow.
pub(crate) fn squared_class(s: i128, r: i128, scale: i128) -> Option<Class> {
    let lo = r.checked_add(-1)?;
    let hi = r.checked_add(1)?;
    let inside = lo.checked_mul(lo)?.checked_mul(scale)?;
    let outside = hi.checked_mul(hi)?.checked_mul(scale)?;
    Some(if s <= inside {
        Class::Inside
    } else if s >= outside {
        Class::Outside
    } else {
        Class::Edge
    })
}
