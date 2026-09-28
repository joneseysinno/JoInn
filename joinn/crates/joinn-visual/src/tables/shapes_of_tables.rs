//! The tables' live rows as shapes, in draw order, as the GPU draws them.

use super::{LIVE, Tables};
use crate::pick::{Geom, PORT_TAG, Shape, WIRE_TAG};

/// Membranes, cells, wires, ports, each by slot. A wire's endpoints are read
/// through `link → incidence → port`, the way the curve organelle reads them.
/// A port's ID carries its cell's generation.
pub fn shapes_of_tables(tables: &Tables) -> Vec<Shape> {
    let live = |flags: u32| flags & LIVE != 0;
    let mut shapes = Vec::new();
    for (slot, m) in (0u32..).zip(&tables.body) {
        if live(m.flags) {
            shapes.push(Shape {
                geom: Geom::RoundRect {
                    x: m.x.into(),
                    y: m.y.into(),
                    w: m.w.into(),
                    h: m.h.into(),
                    r: m.radius.into(),
                },
                id: [slot + 1, 0, 0, m.generation],
            });
        }
    }
    for (slot, c) in (0u32..).zip(&tables.cell) {
        if live(c.flags) {
            shapes.push(Shape {
                geom: Geom::RoundRect {
                    x: c.x.into(),
                    y: c.y.into(),
                    w: c.w.into(),
                    h: c.h.into(),
                    r: c.radius.into(),
                },
                id: [c.body + 1, slot + 1, 0, c.generation],
            });
        }
    }
    let centre = |entry: u32| {
        tables
            .incidence
            .get(entry as usize)
            .and_then(|p| tables.port.get(*p as usize))
            .map(|p| (i64::from(p.x), i64::from(p.y)))
    };
    for (slot, l) in (0u32..).zip(&tables.link) {
        let ends = (centre(l.start), l.start.checked_add(1).and_then(centre));
        if let (true, (Some(a), Some(b))) = (live(l.flags), ends) {
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
    for p in &tables.port {
        let Some(cell) = tables.cell.get(p.cell as usize) else {
            continue;
        };
        if live(p.flags) {
            shapes.push(Shape {
                geom: Geom::Circle {
                    x: p.x.into(),
                    y: p.y.into(),
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
    shapes
}
