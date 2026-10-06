//! A member's stub: from its port's centre out to its gutter.

use joinn_dna::Direction;

use super::{Line, SYSTEM_GUTTER_X};
use crate::charts::Chart;
use crate::layout::{GAP_X, Layout, MARGIN, PortDot};

/// The stub of `port` on `body` (laid out as `layout`, in a system whose root
/// origin is `system`), in root layout units, port end first. It leaves the
/// port horizontally toward its side (in-ports left, out-ports right) and ends
/// on the gutter line on that side of the body's slot (`4 + 48c` or
/// `4 + 48(c + 1)`). When another cell of the body lies across that line it
/// bends: into the column gap beside the port (`GAP_X / 2` from the cell),
/// along it to the nearer margin lane (`MARGIN / 2` from the surface edge),
/// then out to the gutter.
pub(crate) fn stub(body: &Chart, system: (i64, i64), layout: &Layout, port: &PortDot) -> Vec<Line> {
    let (bx, by) = body.root_origin;
    let column = (body.origin.0 - 8).div_euclid(48);
    let out = port.direction == Direction::Out;
    let gutter = system.0 + SYSTEM_GUTTER_X.0 + SYSTEM_GUTTER_X.1 * (column + i64::from(out));
    let (px, py) = (bx + port.x, by + port.y);
    let across = |y: i64, from: i64, to: i64| {
        let (lo, hi) = (from.min(to), from.max(to));
        layout.cells.iter().any(|c| {
            let (cx, cy) = (bx + c.rect.x, by + c.rect.y);
            let own = if out { cx + c.rect.w == px } else { cx == px };
            let own = own && cy <= py && py <= cy + c.rect.h;
            !own && cy <= y && y <= cy + c.rect.h && lo < cx + c.rect.w && cx < hi
        })
    };
    if !across(py, px, gutter) {
        return vec![Line {
            a: (px, py),
            b: (gutter, py),
        }];
    }
    let gap = if out { px + GAP_X / 2 } else { px - GAP_X / 2 };
    let (top, bottom) = (by + MARGIN / 2, by + layout.surface.h - MARGIN / 2);
    let lane = if py - top <= bottom - py { top } else { bottom };
    vec![
        Line {
            a: (px, py),
            b: (gap, py),
        },
        Line {
            a: (gap, py),
            b: (gap, lane),
        },
        Line {
            a: (gap, lane),
            b: (gutter, lane),
        },
    ]
}
