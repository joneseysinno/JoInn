//! A shape's bounding box, in 2^-32 px.

use super::GeomPx;

/// `(x0, y0, x1, y1)`, inclusive. `None` on overflow.
pub(crate) fn bounds_px(g: &GeomPx) -> Option<(i128, i128, i128, i128)> {
    Some(match *g {
        GeomPx::RoundRect { c, h, .. } => (
            c.0.checked_sub(h.0)?,
            c.1.checked_sub(h.1)?,
            c.0.checked_add(h.0)?,
            c.1.checked_add(h.1)?,
        ),
        GeomPx::Circle { c, r } => (
            c.0.checked_sub(r)?,
            c.1.checked_sub(r)?,
            c.0.checked_add(r)?,
            c.1.checked_add(r)?,
        ),
        GeomPx::Capsule { a, b, w } => (
            a.0.min(b.0).checked_sub(w)?,
            a.1.min(b.1).checked_sub(w)?,
            a.0.max(b.0).checked_add(w)?,
            a.1.max(b.1).checked_add(w)?,
        ),
    })
}
