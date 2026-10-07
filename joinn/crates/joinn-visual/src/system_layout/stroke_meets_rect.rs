//! Whether a stroke, with its half-width, meets a closed rectangle. Exact.

use super::{LASSO_HALF_WIDTH, LassoStroke};
use crate::layout::Rect;
use crate::routes::SIXTEENTHS;

/// True when some point of the stroke's centre line is within
/// `LASSO_HALF_WIDTH` sixteenths of `rect` (touching counts). Integer
/// arithmetic in sixteenths, squared distances in `i128`.
pub(super) fn stroke_meets_rect(stroke: &LassoStroke, rect: &Rect) -> bool {
    let s = i128::from(SIXTEENTHS);
    let p = |(x, y): (i64, i64)| (i128::from(x) * s, i128::from(y) * s);
    let (a, b) = (p(stroke.from), p(stroke.to));
    let (x0, y0) = p((rect.x, rect.y));
    let (x1, y1) = p((rect.x + rect.w, rect.y + rect.h));
    let hw = i128::from(LASSO_HALF_WIDTH);
    let inside = |(x, y): (i128, i128)| x0 <= x && x <= x1 && y0 <= y && y <= y1;
    if inside(a) || inside(b) {
        return true;
    }
    let corners = [(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
    let orient = |o: (i128, i128), u: (i128, i128), v: (i128, i128)| {
        ((u.0 - o.0) * (v.1 - o.1) - (u.1 - o.1) * (v.0 - o.0)).signum()
    };
    let on = |o: (i128, i128), u: (i128, i128), q: (i128, i128)| {
        o.0.min(u.0) <= q.0 && q.0 <= o.0.max(u.0) && o.1.min(u.1) <= q.1 && q.1 <= o.1.max(u.1)
    };
    for i in 0..4 {
        let (c, d) = (corners[i], corners[(i + 1) % 4]);
        let (o1, o2, o3, o4) = (
            orient(a, b, c),
            orient(a, b, d),
            orient(c, d, a),
            orient(c, d, b),
        );
        if o1 != o2 && o3 != o4 {
            return true;
        }
        if (o1 == 0 && on(a, b, c))
            || (o2 == 0 && on(a, b, d))
            || (o3 == 0 && on(c, d, a))
            || (o4 == 0 && on(c, d, b))
        {
            return true;
        }
    }
    let point_rect = |(x, y): (i128, i128)| {
        let dx = (x0 - x).max(0).max(x - x1);
        let dy = (y0 - y).max(0).max(y - y1);
        dx * dx + dy * dy <= hw * hw
    };
    if point_rect(a) || point_rect(b) {
        return true;
    }
    let ab = (b.0 - a.0, b.1 - a.1);
    let len2 = ab.0 * ab.0 + ab.1 * ab.1;
    corners.iter().any(|&q| {
        let aq = (q.0 - a.0, q.1 - a.1);
        let dot = aq.0 * ab.0 + aq.1 * ab.1;
        if len2 == 0 || dot <= 0 || dot >= len2 {
            return false;
        }
        let cross = aq.0 * ab.1 - aq.1 * ab.0;
        cross * cross <= hw * hw * len2
    })
}

#[cfg(test)]
mod tests {
    use super::stroke_meets_rect;
    use crate::layout::Rect;
    use crate::system_layout::LassoStroke;

    const R: Rect = Rect {
        x: 10,
        y: 10,
        w: 20,
        h: 10,
    };

    fn meets(from: (i64, i64), to: (i64, i64)) -> bool {
        stroke_meets_rect(&LassoStroke { from, to }, &R)
    }

    #[test]
    fn clear_strokes_do_not_meet_and_touching_ones_do() {
        assert!(!meets((0, 8), (40, 8)), "2 units above");
        assert!(!meets((8, 0), (8, 40)), "2 units left");
        assert!(
            !meets((8, 10), (10, 8)),
            "the corner cut, √2 from the corner"
        );
        assert!(meets((0, 10), (40, 10)), "on the top edge");
        assert!(meets((15, 15), (16, 15)), "inside");
        assert!(meets((0, 0), (40, 40)), "crossing");
        assert!(!meets((31, 15), (40, 15)), "1 unit right is more than ½");
        assert!(meets((30, 15), (40, 15)), "starts on the right edge");
    }
}
