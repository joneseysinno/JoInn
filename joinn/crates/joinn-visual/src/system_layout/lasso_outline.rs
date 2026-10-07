//! The lasso's closed outline around a surface.

use super::{LASSO_CORNER, LassoStroke};
use crate::layout::Rect;

/// Eight straight strokes `gap` units outside `surface`, clockwise from the
/// top side; each corner is cut by a diagonal stroke (no arc, rule 70).
pub fn lasso_outline(surface: &Rect, gap: i64) -> Vec<LassoStroke> {
    let (l, t) = (surface.x - gap, surface.y - gap);
    let (r, b) = (surface.x + surface.w + gap, surface.y + surface.h + gap);
    let c = LASSO_CORNER;
    let points = [
        (l + c, t),
        (r - c, t),
        (r, t + c),
        (r, b - c),
        (r - c, b),
        (l + c, b),
        (l, b - c),
        (l, t + c),
    ];
    (0..points.len())
        .map(|i| LassoStroke {
            from: points[i],
            to: points[(i + 1) % points.len()],
        })
        .collect()
}
