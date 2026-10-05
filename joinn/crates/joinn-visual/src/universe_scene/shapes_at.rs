//! The shapes one camera draws, from the CPU cut.

use joinn_frame::Verdict;

use super::UniverseScene;
use crate::bands::Band;
use crate::camera::Camera;
use crate::lens_cut::{CutForm, cut};
use crate::pick::{GALAXY_TAG, Geom, PORT_TAG, SYSTEM_TAG, Shape, WIRE_TAG};
use crate::refuse::refuse;
use crate::tables::{CHART_BODY, CHART_GALAXY, LIVE, SURFACE_PORT};

impl UniverseScene {
    /// What owns pixels under `camera`, in the GPU's draw order: frames, dots,
    /// surfaces, cells, wires, ports, strokes, each by slot. A frame or node is
    /// drawn when its chart is in the cut; a body's elements by its owner band
    /// (dot: the dot; glyph: the surface; summary: the surface, cells and
    /// surface ports; full: everything, with text). Coordinates are relative to
    /// the anchor. The camera must be anchored where the chart rows are.
    pub fn shapes_at(&self, camera: &Camera) -> Verdict<Vec<Shape>> {
        if camera.anchor != self.anchor {
            return refuse(format!(
                "pick: the camera is anchored at chart {} and the chart rows at chart {}; acceptance is a camera rebased onto the scene's anchor",
                camera.anchor.0, self.anchor.0
            ));
        }
        let t = &self.tables;
        let mut forms: Vec<Option<CutForm>> = vec![None; t.chart.len()];
        for e in cut(&self.layout, camera).entries {
            if let Some(f) = self
                .chart_slot(e.chart)
                .and_then(|s| forms.get_mut(s as usize))
            {
                *f = Some(e.form);
            }
        }
        let form = |slot: u32| forms.get(slot as usize).copied().flatten();
        let band = |slot: u32| match form(slot) {
            Some(CutForm::Drawn(b)) => Some(b),
            _ => None,
        };
        let origin = |slot: u32| {
            t.chart
                .get(slot as usize)
                .map_or((0, 0), |c| (i64::from(c.origin_x), i64::from(c.origin_y)))
        };
        let live = |flags: u32| flags & LIVE != 0;
        let mut shapes = Vec::new();
        for f in &t.frame {
            if live(f.flags) && form(f.chart).is_some() {
                let (x, y) = origin(f.chart);
                let tag = if f.kind == CHART_GALAXY {
                    GALAXY_TAG
                } else {
                    SYSTEM_TAG
                };
                shapes.push(Shape {
                    geom: Geom::RoundRect {
                        x,
                        y,
                        w: f.w.into(),
                        h: f.h.into(),
                        r: 0,
                    },
                    id: [0, 0, tag | f.index, f.generation],
                });
            }
        }
        for (slot, m) in (0u32..).zip(&t.body) {
            if live(m.flags) && band(slot) == Some(Band::Dot) {
                let (x, y) = origin(slot);
                shapes.push(Shape {
                    geom: Geom::Dot {
                        x: 16 * (x + i64::from(m.x)) + 8 * i64::from(m.w),
                        y: 16 * (y + i64::from(m.y)) + 8 * i64::from(m.h),
                    },
                    id: [slot + 1, 0, 0, m.generation],
                });
            }
        }
        for (slot, m) in (0u32..).zip(&t.body) {
            if live(m.flags) && band(slot) > Some(Band::Dot) {
                let (x, y) = origin(slot);
                shapes.push(Shape {
                    geom: Geom::RoundRect {
                        x: x + i64::from(m.x),
                        y: y + i64::from(m.y),
                        w: m.w.into(),
                        h: m.h.into(),
                        r: m.radius.into(),
                    },
                    id: [slot + 1, 0, 0, m.generation],
                });
            }
        }
        for (slot, c) in (0u32..).zip(&t.cell) {
            if live(c.flags) && band(c.body) >= Some(Band::Summary) {
                let (x, y) = origin(c.body);
                shapes.push(Shape {
                    geom: Geom::RoundRect {
                        x: x + i64::from(c.x),
                        y: y + i64::from(c.y),
                        w: c.w.into(),
                        h: c.h.into(),
                        r: c.radius.into(),
                    },
                    id: [c.body + 1, slot + 1, 0, c.generation],
                });
            }
        }
        let centre = |entry: u32, o: (i64, i64)| {
            t.incidence
                .get(entry as usize)
                .and_then(|p| t.port.get(*p as usize))
                .map(|p| (o.0 + i64::from(p.x), o.1 + i64::from(p.y)))
        };
        for (slot, l) in (0u32..).zip(&t.link) {
            if !live(l.flags) || band(l.body) != Some(Band::Full) {
                continue;
            }
            let o = origin(l.body);
            let ends = (
                centre(l.start, o),
                l.start.checked_add(1).and_then(|e| centre(e, o)),
            );
            if let (Some(a), Some(b)) = ends {
                shapes.push(Shape {
                    geom: Geom::Capsule {
                        a,
                        b,
                        half_quarters: l.half_width_quarters.into(),
                    },
                    id: [l.body + 1, 0, WIRE_TAG | slot, l.generation],
                });
            }
        }
        for p in &t.port {
            let Some(cell) = t.cell.get(p.cell as usize) else {
                continue;
            };
            let least = if p.flags & SURFACE_PORT != 0 {
                Band::Summary
            } else {
                Band::Full
            };
            if live(p.flags) && band(cell.body) >= Some(least) {
                let (x, y) = origin(cell.body);
                shapes.push(Shape {
                    geom: Geom::Circle {
                        x: x + i64::from(p.x),
                        y: y + i64::from(p.y),
                        r: p.radius.into(),
                    },
                    id: [
                        cell.body + 1,
                        p.cell + 1,
                        PORT_TAG | p.position,
                        cell.generation,
                    ],
                });
            }
        }
        for s in &t.stroke {
            let on_body = t.chart.get(s.chart as usize).map(|c| c.kind) == Some(CHART_BODY);
            let drawn = if on_body {
                band(s.chart) == Some(Band::Full)
            } else {
                form(s.chart).is_some()
            };
            if live(s.flags) && drawn {
                let (x, y) = origin(s.chart);
                let (x, y) = (16 * x, 16 * y);
                shapes.push(Shape {
                    geom: Geom::Stroke {
                        a: (x + i64::from(s.x0), y + i64::from(s.y0)),
                        b: (x + i64::from(s.x1), y + i64::from(s.y1)),
                        half: s.half_width.into(),
                    },
                    id: [s.owner[0], s.owner[1], s.owner[2], s.generation],
                });
            }
        }
        Verdict::Ok(shapes)
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use std::collections::BTreeSet;

    use crate::camera::{Camera, ChartId, LEVEL_MAX};
    use crate::fixtures::phase5_universe;
    use crate::layout::Rect;
    use crate::pick::{Pick, cpu_pick_at, cpu_pick_reference_at};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn the_grid_and_the_reference_agree_on_every_pixel_from_the_galaxy_to_level_9() {
        let layout = phase5_universe();
        let galaxy = Rect {
            x: 64,
            y: 64,
            w: 1296,
            h: 704,
        };
        let (w, h) = (320, 180);
        let pin = (i64::from(w / 2), i64::from(h / 2));
        let mut camera = layout.rebase(Camera::frame(galaxy, ChartId(0), w, h));
        let mut seen = 0;
        loop {
            let scene = match UniverseScene::grow(layout.clone(), camera.anchor) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let shapes = match scene.shapes_at(&camera) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let (Verdict::Ok(grid), Verdict::Ok(reference)) = (
                cpu_pick_at(&shapes, &camera),
                cpu_pick_reference_at(&shapes, &camera),
            ) else {
                panic!("both pickers must run at {:?}", camera.zoom);
            };
            assert_eq!(grid, reference, "{:?}", camera.zoom);
            let owners: BTreeSet<[u32; 4]> = grid
                .pixels
                .iter()
                .filter_map(|p| match p {
                    Pick::Owned(id) => Some(*id),
                    _ => None,
                })
                .collect();
            assert!(
                !owners.is_empty(),
                "{:?}: nothing owns a pixel",
                camera.zoom
            );
            seen += 1;
            if camera.zoom.level == LEVEL_MAX {
                break;
            }
            camera = match camera.zoom_about(pin, 4) {
                Verdict::Ok(c) => layout.rebase(c),
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
        }
        assert!(seen > 10, "{seen} zooms");
    }

    #[test]
    fn a_camera_anchored_elsewhere_is_refused() {
        let layout = phase5_universe();
        let scene = match UniverseScene::grow(layout, ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = Camera::frame(
            Rect {
                x: 0,
                y: 0,
                w: 64,
                h: 64,
            },
            ChartId(1),
            320,
            180,
        );
        assert!(matches!(scene.shapes_at(&camera), Verdict::Refused(_)));
    }
}
