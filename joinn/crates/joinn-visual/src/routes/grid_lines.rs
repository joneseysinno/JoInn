//! The streets of one fold state, before they are split into a graph.

use super::{
    Fold, GALAXY_GUTTER_X, GALAXY_GUTTER_Y, GALAXY_REACH, GALAXY_SIDES, Line, SYSTEM_GUTTER_X,
    SYSTEM_GUTTER_Y, SYSTEM_REACH, SYSTEM_SIDES, UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y,
};
use crate::charts::{ChartKind, UniverseLayout};

/// The universe's grid lines, each from its container's first line to its last
/// in the other direction; every open galaxy's and (when open) system's grid
/// lines with their extensions to the parent's lines; and for every folded
/// system or galaxy only the four extension pieces from its touch points out
/// to the parent's lines. A folded node's interior has no street.
pub fn grid_lines(layout: &UniverseLayout, fold: Fold) -> Vec<Line> {
    let mut lines = Vec::new();
    let span = |(first, step, n): (i64, i64, i64)| (first, first + step * (n - 1));
    let grid = |lines: &mut Vec<Line>,
                origin: (i64, i64),
                xs: (i64, i64, i64),
                ys: (i64, i64, i64),
                reach: Option<((i64, i64), (i64, i64))>| {
        let ((x_lo, x_hi), (y_lo, y_hi)) = reach.unwrap_or((span(xs), span(ys)));
        for c in 0..xs.2 {
            let x = origin.0 + xs.0 + xs.1 * c;
            lines.push(Line {
                a: (x, origin.1 + y_lo),
                b: (x, origin.1 + y_hi),
            });
        }
        for r in 0..ys.2 {
            let y = origin.1 + ys.0 + ys.1 * r;
            lines.push(Line {
                a: (origin.0 + x_lo, y),
                b: (origin.0 + x_hi, y),
            });
        }
    };
    let sides = |lines: &mut Vec<Line>,
                 origin: (i64, i64),
                 points: [(i64, i64); 4],
                 ((x_lo, x_hi), (y_lo, y_hi)): ((i64, i64), (i64, i64))| {
        let [left, right, top, bottom] = points;
        let at = |p: (i64, i64)| (origin.0 + p.0, origin.1 + p.1);
        for (from, to) in [
            (left, (x_lo, left.1)),
            (right, (x_hi, right.1)),
            (top, (top.0, y_lo)),
            (bottom, (bottom.0, y_hi)),
        ] {
            lines.push(Line {
                a: at(to.min(from)),
                b: at(to.max(from)),
            });
        }
    };
    grid(
        &mut lines,
        (0, 0),
        UNIVERSE_GUTTER_X,
        UNIVERSE_GUTTER_Y,
        None,
    );
    for chart in &layout.charts {
        match (chart.kind, fold) {
            (ChartKind::Galaxy, Fold::Galaxies) => {
                sides(&mut lines, chart.root_origin, GALAXY_SIDES, GALAXY_REACH);
            }
            (ChartKind::Galaxy, _) => grid(
                &mut lines,
                chart.root_origin,
                GALAXY_GUTTER_X,
                GALAXY_GUTTER_Y,
                Some(GALAXY_REACH),
            ),
            (ChartKind::System, Fold::Open) => grid(
                &mut lines,
                chart.root_origin,
                SYSTEM_GUTTER_X,
                SYSTEM_GUTTER_Y,
                Some(SYSTEM_REACH),
            ),
            (ChartKind::System, Fold::Systems) => {
                sides(&mut lines, chart.root_origin, SYSTEM_SIDES, SYSTEM_REACH);
            }
            _ => {}
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use crate::charts::{BODY_MAX, GALAXY_SIZE, SYSTEM_SIZE};
    use crate::routes::{
        GALAXY_GUTTER_X, GALAXY_GUTTER_Y, GALAXY_REACH, SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y,
        SYSTEM_REACH, UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y,
    };

    type Grid = ((i64, i64, i64), (i64, i64, i64));

    fn values((first, step, n): (i64, i64, i64)) -> Vec<i64> {
        (0..n).map(|i| first + step * i).collect()
    }

    /// Half-open slot rectangles `(x, y, w, h)`, `across` per row.
    fn slots(
        n: i64,
        across: i64,
        x0: i64,
        dx: i64,
        y0: i64,
        dy: i64,
        size: (i64, i64),
    ) -> Vec<(i64, i64, i64, i64)> {
        (0..n)
            .map(|i| {
                (
                    x0 + dx * (i % across),
                    y0 + dy * (i / across),
                    size.0,
                    size.1,
                )
            })
            .collect()
    }

    /// No line of the grid, over its span, meets any slot.
    fn misses(grid: Grid, slots: &[(i64, i64, i64, i64)]) -> Vec<String> {
        let (xs, ys) = (values(grid.0), values(grid.1));
        let (x_span, y_span) = ((xs[0], xs[xs.len() - 1]), (ys[0], ys[ys.len() - 1]));
        let mut hits = Vec::new();
        for &(sx, sy, w, h) in slots {
            for &x in &xs {
                if sx <= x && x < sx + w && y_span.0 < sy + h && sy <= y_span.1 {
                    hits.push(format!("x = {x} meets slot at ({sx}, {sy})"));
                }
            }
            for &y in &ys {
                if sy <= y && y < sy + h && x_span.0 < sx + w && sx <= x_span.1 {
                    hits.push(format!("y = {y} meets slot at ({sx}, {sy})"));
                }
            }
        }
        hits
    }

    #[test]
    fn every_grid_line_misses_every_slot_of_its_level() {
        let bodies = slots(24, 6, 8, 48, 16, 32, BODY_MAX);
        let systems = slots(16, 4, 16, 320, 32, 168, SYSTEM_SIZE);
        let galaxies = slots(8, 4, 64, 1360, 64, 768, GALAXY_SIZE);
        assert!(misses((SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y), &bodies).is_empty());
        assert!(misses((GALAXY_GUTTER_X, GALAXY_GUTTER_Y), &systems).is_empty());
        assert!(misses((UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y), &galaxies).is_empty());
        // A line moved four units into the bodies meets them.
        let moved = ((8, 48, 7), SYSTEM_GUTTER_Y);
        assert!(!misses(moved, &bodies).is_empty());
    }

    #[test]
    fn every_extension_ends_on_a_parent_line() {
        let on_parent = |p: (i64, i64), parent: Grid| {
            let (xs, ys) = (values(parent.0), values(parent.1));
            let (x_span, y_span) = ((xs[0], xs[xs.len() - 1]), (ys[0], ys[ys.len() - 1]));
            (xs.contains(&p.0) && y_span.0 <= p.1 && p.1 <= y_span.1)
                || (ys.contains(&p.1) && x_span.0 <= p.0 && p.0 <= x_span.1)
        };
        let ends = |grid: Grid, ((x_lo, x_hi), (y_lo, y_hi)): ((i64, i64), (i64, i64))| {
            let mut out = Vec::new();
            for y in values(grid.1) {
                out.extend([(x_lo, y), (x_hi, y)]);
            }
            for x in values(grid.0) {
                out.extend([(x, y_lo), (x, y_hi)]);
            }
            out
        };
        let mut misses = Vec::new();
        for (sx, sy, _, _) in slots(16, 4, 16, 320, 32, 168, SYSTEM_SIZE) {
            for (x, y) in ends((SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y), SYSTEM_REACH) {
                if !on_parent((sx + x, sy + y), (GALAXY_GUTTER_X, GALAXY_GUTTER_Y)) {
                    misses.push(format!("system at ({sx}, {sy}): ({x}, {y})"));
                }
            }
        }
        for (gx, gy, _, _) in slots(8, 4, 64, 1360, 64, 768, GALAXY_SIZE) {
            for (x, y) in ends((GALAXY_GUTTER_X, GALAXY_GUTTER_Y), GALAXY_REACH) {
                if !on_parent((gx + x, gy + y), (UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y)) {
                    misses.push(format!("galaxy at ({gx}, {gy}): ({x}, {y})"));
                }
            }
        }
        assert!(misses.is_empty(), "{misses:?}");
    }
}
