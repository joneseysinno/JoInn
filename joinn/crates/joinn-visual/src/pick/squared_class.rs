//! Classify a squared distance against a radius, without a square root.

use super::{Class, EDGE};

/// `s` against `(r − EDGE)²` and `(r + EDGE)²`. A radius under the edge band
/// holds nothing inside. `None` on overflow.
pub(crate) fn squared_class(s: i128, r: i128) -> Option<Class> {
    let lo = r.checked_sub(EDGE)?;
    let hi = r.checked_add(EDGE)?;
    let inside = lo >= 0 && s <= lo.checked_mul(lo)?;
    let outside = s >= hi.checked_mul(hi)?;
    Some(if inside {
        Class::Inside
    } else if outside {
        Class::Outside
    } else {
        Class::Edge
    })
}
