//! Classify a point against a shape already put through the camera.

use super::arrow_class::arrow_class;
use super::capsule_class::capsule_class;
use super::circle_class::circle_class;
use super::round_rect_class::round_rect_class;
use super::{Class, GeomPx};

/// `p` in 2^-32 px. `None` on overflow.
pub(crate) fn class_at(g: &GeomPx, p: (i128, i128)) -> Option<Class> {
    match *g {
        GeomPx::RoundRect { c, h, r } => round_rect_class(p, c, h, r),
        GeomPx::Circle { c, r } => circle_class(p, c, r),
        GeomPx::Capsule { a, b, w } => capsule_class(p, a, b, w),
        GeomPx::Arrow { a, b, w } => arrow_class(p, a, b, w),
    }
}
