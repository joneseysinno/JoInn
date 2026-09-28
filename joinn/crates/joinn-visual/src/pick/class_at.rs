//! Classify a point against a shape already put through the camera.

use super::capsule_class::capsule_class;
use super::circle_class::circle_class;
use super::round_rect_class::round_rect_class;
use super::{Class, Geom16};

/// `p` in sixteenths of a pixel. `None` on overflow.
pub(crate) fn class_at(g: &Geom16, p: (i128, i128)) -> Option<Class> {
    match *g {
        Geom16::RoundRect { c, h, r } => round_rect_class(p, c, h, r),
        Geom16::Circle { c, r } => circle_class(p, c, r),
        Geom16::Capsule { a, b, w } => capsule_class(p, a, b, w),
    }
}
