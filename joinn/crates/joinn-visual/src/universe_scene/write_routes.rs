//! Write every route's row and its segments' rows (plan 7.3 §2.7).

use joinn_frame::Verdict;

use super::UniverseScene;
use super::fit::fit;
use crate::charts::ChartKind;
use crate::routes::SIXTEENTHS;
use crate::tables::{LIVE, RouteRow, Row, STYLE_HUB, STYLE_SPINE, SegmentRow};

impl UniverseScene {
    /// Route rows in the layout's order (fold state, then link); each route's
    /// segments in piece order after the previous route's. A segment lives in
    /// the deepest system, else galaxy, else the universe, whose closed
    /// rectangle holds both its ends, in sixteenths of that chart, so a rebase
    /// moves it as it moves the chart. Spines take the spine style, every other
    /// piece the hub style (the shader draws a region in its own).
    pub(super) fn write_routes(&mut self) -> Verdict<()> {
        let s = SIXTEENTHS;
        let holds = |kind: ChartKind, a: (i64, i64), b: (i64, i64)| {
            self.layout.charts.iter().position(|c| {
                let (x0, y0) = (c.root_origin.0 * s, c.root_origin.1 * s);
                let (x1, y1) = (x0 + c.size.0 * s, y0 + c.size.1 * s);
                let inside = |p: (i64, i64)| p.0 >= x0 && p.0 <= x1 && p.1 >= y0 && p.1 <= y1;
                c.kind == kind && inside(a) && inside(b)
            })
        };
        let mut rows: Vec<(RouteRow, Vec<SegmentRow>)> = Vec::new();
        for route in &self.layout.routes.routes {
            let style = if route.ordered {
                STYLE_SPINE
            } else {
                STYLE_HUB
            };
            let mut segments = Vec::new();
            for p in &route.pieces {
                let chart = holds(ChartKind::System, p.a, p.b)
                    .or_else(|| holds(ChartKind::Galaxy, p.a, p.b))
                    .unwrap_or(0);
                let (Some(c), Some(&slot)) = (self.layout.charts.get(chart), self.slots.get(chart))
                else {
                    continue;
                };
                let (ox, oy) = (c.root_origin.0 * s, c.root_origin.1 * s);
                segments.push(SegmentRow {
                    chart: slot,
                    x0: match fit(p.a.0 - ox) {
                        Verdict::Ok(v) => v,
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    },
                    y0: match fit(p.a.1 - oy) {
                        Verdict::Ok(v) => v,
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    },
                    x1: match fit(p.b.0 - ox) {
                        Verdict::Ok(v) => v,
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    },
                    y1: match fit(p.b.1 - oy) {
                        Verdict::Ok(v) => v,
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    },
                    route: 0,
                    member: p.member,
                    kind: p.kind.number(),
                    legs: p.legs,
                    style,
                    generation: 1,
                    flags: LIVE,
                });
            }
            let numbers = (
                fit::<u32>(i64::try_from(route.link).unwrap_or(-1)),
                fit::<u32>(route.size),
                fit::<u32>(i64::try_from(segments.len()).unwrap_or(-1)),
            );
            let (link, size, count) = match numbers {
                (Verdict::Ok(l), Verdict::Ok(z), Verdict::Ok(n)) => (l, z, n),
                (Verdict::Refused(r), _, _)
                | (_, Verdict::Refused(r), _)
                | (_, _, Verdict::Refused(r)) => return Verdict::Refused(r),
            };
            let row = RouteRow {
                link,
                fold: route.fold.number(),
                size,
                ordered: u32::from(route.ordered),
                segment_first: 0,
                segment_count: count,
                generation: 1,
                flags: LIVE,
            };
            rows.push((row, segments));
        }
        let mut first = 0u32;
        for (slot, (mut row, segments)) in (0u32..).zip(rows) {
            row.segment_first = first;
            self.tables.put(slot, Row::Route(row));
            for mut seg in segments {
                seg.route = slot;
                self.tables.put(first, Row::Segment(seg));
                first += 1;
            }
        }
        Verdict::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::ChartId;
    use crate::fixtures::phase5_universe;
    use crate::routes::{Fold, PieceKind};
    use crate::tables::{CHART_SYSTEM, STYLE_SPINE};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn the_phase_5_link_has_a_route_per_fold_state_and_a_segment_per_piece() {
        let layout = phase5_universe();
        let routes = layout.routes.clone();
        let scene = match UniverseScene::grow(layout, ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let t = scene.tables();
        assert_eq!(routes.folds, [Fold::Open, Fold::Systems]);
        assert_eq!(t.route.len(), 2);
        let pieces: Vec<usize> = routes.routes.iter().map(|r| r.pieces.len()).collect();
        let counts: Vec<usize> = t.route.iter().map(|r| r.segment_count as usize).collect();
        assert_eq!(counts, pieces);
        assert_eq!(t.segment.len(), pieces.iter().sum::<usize>());
        assert_eq!((t.route[0].fold, t.route[1].fold), (0, 1));
        assert_eq!(t.route[1].segment_first, t.route[0].segment_count);
        assert!(t.route.iter().all(|r| r.ordered == 1 && r.link == 0));
        assert!(t.segment.iter().all(|s| s.style == STYLE_SPINE));
        // Every open stub runs from its port to its system's gutter: it lives
        // in that system's chart, its ends in that chart's sixteenths.
        let open = &routes.routes[0];
        let mut stubs = 0;
        for (p, seg) in open.pieces.iter().zip(&t.segment) {
            if p.kind != PieceKind::Stub {
                continue;
            }
            stubs += 1;
            assert_eq!(t.chart[seg.chart as usize].kind, CHART_SYSTEM);
            let at = (0u32..)
                .zip(&scene.layout().charts)
                .find(|(i, _)| scene.chart_slot(ChartId(*i)) == Some(seg.chart))
                .map(|(_, c)| (c.root_origin.0 * 16, c.root_origin.1 * 16))
                .unwrap_or_else(|| panic!("segment chart {} is laid out", seg.chart));
            assert_eq!(
                (i64::from(seg.x0) + at.0, i64::from(seg.y0) + at.1),
                p.a,
                "{p:?}"
            );
            assert_eq!((i64::from(seg.x1) + at.0, i64::from(seg.y1) + at.1), p.b);
        }
        assert_eq!(stubs, 2);
    }
}
