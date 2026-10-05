//! Grow a universe scene from a laid-out lens.

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::Direction;
use joinn_frame::Verdict;
use joinn_link::Address;

use super::UniverseScene;
use super::fit::fit;
use crate::camera::ChartId;
use crate::charts::{ChartKind, UniverseLayout};
use crate::font::{TitleOf, cell_label, title};
use crate::layout::{CELL_RADIUS, PORT_RADIUS, SURFACE_RADIUS, WIRE_HALF_WIDTH_QUARTERS};
use crate::pick::{GALAXY_TAG, SYSTEM_TAG};
use crate::tables::{
    BodyRow, CHART_GALAXY, CHART_SYSTEM, CellRow, FrameRow, LIVE, LinkRow, PortRow, Row,
    STYLE_CELL, STYLE_TEXT, SURFACE_PORT, StrokeRow, Tables, WIRE_KIND,
};

macro_rules! take {
    ($v:expr) => {
        match $v {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    };
}

impl UniverseScene {
    /// Charts relative to `anchor`; then per body in body order its surface,
    /// its cells by name, their ports by position (surface ports flagged), and
    /// its wires; then frames in chart order; then strokes: each frame's
    /// title, then each body's cell labels. Values come later, from `present`.
    /// Nothing is pending.
    pub fn grow(layout: UniverseLayout, anchor: ChartId) -> Verdict<UniverseScene> {
        let count = |k: ChartKind| layout.charts.iter().filter(|c| c.kind == k).count();
        let (bodies, systems, galaxies) = (
            count(ChartKind::Body),
            count(ChartKind::System),
            count(ChartKind::Galaxy),
        );
        let mut slots = Vec::with_capacity(layout.charts.len());
        for c in &layout.charts {
            let base = match c.kind {
                ChartKind::Body => 0,
                ChartKind::System => bodies,
                ChartKind::Galaxy => bodies + systems,
                ChartKind::Universe => bodies + systems + galaxies,
            };
            let index = if c.kind == ChartKind::Universe {
                0
            } else {
                c.index as usize
            };
            slots.push(take!(fit::<u32>(
                i64::try_from(base + index).unwrap_or(i64::MAX)
            )));
        }
        let mut scene = UniverseScene {
            layout,
            tables: Tables::empty(),
            anchor,
            slots,
            bodies: BTreeMap::new(),
            cells: BTreeMap::new(),
            ports: BTreeMap::new(),
            values: BTreeMap::new(),
            value_start: 0,
            pending: BTreeSet::new(),
        };
        take!(scene.write_charts());

        let charts = scene.layout.charts.clone();
        let mut labels: Vec<StrokeRow> = Vec::new();
        for c in charts.iter().filter(|c| c.kind == ChartKind::Body) {
            let Some((_, li)) = c.body else { continue };
            let Some(l) = scene.layout.layouts.get(li).cloned() else {
                continue;
            };
            let body = c.index;
            scene.bodies.insert(c.name.clone(), body);
            scene.tables.put(
                body,
                Row::Body(BodyRow {
                    x: take!(fit(l.surface.x)),
                    y: take!(fit(l.surface.y)),
                    w: take!(fit(l.surface.w)),
                    h: take!(fit(l.surface.h)),
                    radius: take!(fit(SURFACE_RADIUS)),
                    generation: 1,
                    flags: LIVE,
                }),
            );
            for cell in &l.cells {
                let slot = take!(fit::<u32>(
                    i64::try_from(scene.tables.cell.len()).unwrap_or(-1)
                ));
                scene.tables.put(
                    slot,
                    Row::Cell(CellRow {
                        body,
                        x: take!(fit(cell.rect.x)),
                        y: take!(fit(cell.rect.y)),
                        w: take!(fit(cell.rect.w)),
                        h: take!(fit(cell.rect.h)),
                        radius: take!(fit(CELL_RADIUS)),
                        generation: 1,
                        style: STYLE_CELL,
                        flags: LIVE,
                    }),
                );
                scene.cells.insert((body, cell.instance.clone()), slot);
                for s in take!(cell_label(cell)) {
                    labels.push(StrokeRow {
                        chart: body,
                        x0: take!(fit(s.x0)),
                        y0: take!(fit(s.y0)),
                        x1: take!(fit(s.x1)),
                        y1: take!(fit(s.y1)),
                        half_width: take!(fit(s.half_width)),
                        owner: [body + 1, slot + 1, 0],
                        style: STYLE_TEXT,
                        generation: 1,
                        flags: LIVE,
                    });
                }
            }
            let consumed: BTreeSet<&Address> =
                l.wires.iter().flat_map(|w| [&w.src, &w.dst]).collect();
            for p in &l.ports {
                let Some(&cell) = scene.cells.get(&(body, p.address.instance.clone())) else {
                    continue;
                };
                let slot = take!(fit::<u32>(
                    i64::try_from(scene.tables.port.len()).unwrap_or(-1)
                ));
                let surface = if consumed.contains(&p.address) {
                    0
                } else {
                    SURFACE_PORT
                };
                scene.tables.put(
                    slot,
                    Row::Port(PortRow {
                        cell,
                        position: p.address.port,
                        direction: match p.direction {
                            Direction::In => 0,
                            Direction::Out => 1,
                        },
                        x: take!(fit(p.x)),
                        y: take!(fit(p.y)),
                        radius: take!(fit(PORT_RADIUS)),
                        generation: 1,
                        flags: LIVE | surface,
                    }),
                );
                scene.ports.insert((body, p.address.clone()), slot);
            }
            for w in &l.wires {
                let (Some(&src), Some(&dst)) = (
                    scene.ports.get(&(body, w.src.clone())),
                    scene.ports.get(&(body, w.dst.clone())),
                ) else {
                    continue;
                };
                let slot = take!(fit::<u32>(
                    i64::try_from(scene.tables.link.len()).unwrap_or(-1)
                ));
                let start = slot * 2;
                scene.tables.put(
                    slot,
                    Row::Link(LinkRow {
                        kind: WIRE_KIND,
                        start,
                        count: 2,
                        body,
                        half_width_quarters: take!(fit(WIRE_HALF_WIDTH_QUARTERS)),
                        generation: 1,
                        flags: LIVE,
                    }),
                );
                scene.tables.put(start, Row::Incidence(src));
                scene.tables.put(start + 1, Row::Incidence(dst));
            }
        }

        let mut titles: Vec<StrokeRow> = Vec::new();
        for (i, c) in charts.iter().enumerate() {
            let (kind, tag, of) = match c.kind {
                ChartKind::Galaxy => (CHART_GALAXY, GALAXY_TAG, TitleOf::Galaxy),
                ChartKind::System => (CHART_SYSTEM, SYSTEM_TAG, TitleOf::System),
                _ => continue,
            };
            let Some(&chart) = scene.slots.get(i) else {
                continue;
            };
            let slot = take!(fit::<u32>(
                i64::try_from(scene.tables.frame.len()).unwrap_or(-1)
            ));
            scene.tables.put(
                slot,
                Row::Frame(FrameRow {
                    chart,
                    w: take!(fit(c.size.0)),
                    h: take!(fit(c.size.1)),
                    kind,
                    index: c.index,
                    generation: 1,
                    flags: LIVE,
                }),
            );
            for s in take!(title(&c.name, of)) {
                titles.push(StrokeRow {
                    chart,
                    x0: take!(fit(s.x0)),
                    y0: take!(fit(s.y0)),
                    x1: take!(fit(s.x1)),
                    y1: take!(fit(s.y1)),
                    half_width: take!(fit(s.half_width)),
                    owner: [0, 0, tag | c.index],
                    style: STYLE_TEXT,
                    generation: 1,
                    flags: LIVE,
                });
            }
        }
        for (slot, row) in (0u32..).zip(titles.into_iter().chain(labels)) {
            scene.tables.put(slot, Row::Stroke(row));
        }
        scene.value_start = take!(fit(i64::try_from(scene.tables.stroke.len()).unwrap_or(-1)));
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::ChartId;
    use crate::fixtures::phase5_universe;
    use crate::tables::{CHART_BODY, CHART_UNIVERSE, SURFACE_PORT};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn the_phase_5_universe_grows_bodies_first_then_systems_galaxies_and_the_universe() {
        let scene = match UniverseScene::grow(phase5_universe(), ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let t = scene.tables();
        let kinds: Vec<u32> = t.chart.iter().map(|c| c.kind).collect();
        assert_eq!(kinds, [3, 3, 2, 2, 1, 0]);
        assert_eq!(t.chart[0].kind, CHART_BODY);
        assert_eq!((t.chart[0].origin_x, t.chart[0].origin_y), (88, 112));
        assert_eq!(t.chart[0].parent, 2, "calc's system is chart slot 2");
        assert_eq!(t.chart[5].kind, CHART_UNIVERSE);
        assert_eq!(t.chart[5].parent, 5, "the root is its own parent");
        assert_eq!(t.body.len(), 2);
        assert_eq!(t.frame.len(), 3, "one galaxy and two systems");
        assert!(t.port.iter().any(|p| p.flags & SURFACE_PORT != 0));
        assert!(t.port.iter().any(|p| p.flags & SURFACE_PORT == 0));
        assert!(!t.stroke.is_empty());
        assert_eq!(scene.body_slot("units"), Some(1));
    }
}
