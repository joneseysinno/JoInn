//! The lasso's rules (plan 7.4 §2.8), checked exactly in layout units.

use joinn_frame::Verdict;

use super::SystemLayout;
use super::stroke_meets_rect::stroke_meets_rect;
use crate::layout::Rect;
use crate::refuse::refuse;

/// 1. No stroke, with its half-width, meets the surface or a grown cell or
///    waiting box; no outline stroke meets the response.
/// 2. Every grown cell and waiting box lies strictly inside the outline.
/// 3. The response lies outside the outline, and the arrowhead's tip is on
///    its in-port side.
///
/// Rule 4 (the rules hold after every growth step) is this check at every size.
pub fn check_lasso(layout: &SystemLayout) -> Verdict<()> {
    let outline_len = 8;
    if layout.lasso.len() != outline_len + 3 {
        return refuse(format!(
            "lasso: {} strokes; acceptance is an outline of 8, a neck and an arrowhead of 2",
            layout.lasso.len()
        ));
    }
    let body: Vec<(&str, &Rect)> = layout
        .cells
        .iter()
        .chain(&layout.waiting)
        .map(|c| (c.instance.as_str(), &c.rect))
        .collect();
    for (i, stroke) in layout.lasso.iter().enumerate() {
        let mut against: Vec<(&str, &Rect)> = vec![("the surface", &layout.surface)];
        against.extend(body.iter().copied());
        if i < outline_len {
            against.push((layout.response.instance.as_str(), &layout.response.rect));
        }
        if let Some((what, _)) = against.iter().find(|(_, r)| stroke_meets_rect(stroke, r)) {
            return refuse(format!(
                "lasso: stroke {i} meets {what}; acceptance is a lasso clear of every cell and the surface (rule 1)"
            ));
        }
    }
    let outline: Vec<((i64, i64), (i64, i64))> = layout.lasso[..outline_len]
        .iter()
        .map(|s| (s.from, s.to))
        .collect();
    let inside = |(x, y): (i64, i64)| {
        outline.iter().all(|&((ax, ay), (bx, by))| {
            i128::from(bx - ax) * i128::from(y - ay) - i128::from(by - ay) * i128::from(x - ax) > 0
        })
    };
    for (name, r) in &body {
        let corners = [
            (r.x, r.y),
            (r.x + r.w, r.y),
            (r.x + r.w, r.y + r.h),
            (r.x, r.y + r.h),
        ];
        if !corners.into_iter().all(inside) {
            return refuse(format!(
                "lasso: {name} is not inside the outline; acceptance is every grown cell and waiting box inside (rule 2)"
            ));
        }
    }
    let xs = outline.iter().map(|(a, _)| a.0);
    let ys = outline.iter().map(|(a, _)| a.1);
    let (min_x, max_x) = (xs.clone().min().unwrap_or(0), xs.max().unwrap_or(0));
    let (min_y, max_y) = (ys.clone().min().unwrap_or(0), ys.max().unwrap_or(0));
    let resp = &layout.response.rect;
    let apart =
        resp.x > max_x || resp.x + resp.w < min_x || resp.y > max_y || resp.y + resp.h < min_y;
    if !apart {
        return refuse(format!(
            "lasso: the response {} is not outside the outline; acceptance is the response outside (rule 3)",
            layout.response.instance
        ));
    }
    let (tx, ty) = layout.tip;
    let on_side = tx == resp.x && resp.y <= ty && ty <= resp.y + resp.h;
    let neck_ends = layout.lasso[outline_len].to == layout.tip;
    if !on_side || !neck_ends {
        return refuse(format!(
            "lasso: the arrowhead's tip ({tx}, {ty}) is not on {}'s in-port side; acceptance is the tip on it (rule 3)",
            layout.response.instance
        ));
    }
    Verdict::Ok(())
}
