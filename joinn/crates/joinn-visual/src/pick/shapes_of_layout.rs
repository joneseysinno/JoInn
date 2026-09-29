//! A fresh layout's shapes in draw order, with the IDs of canonical growth.

use super::{Geom, PORT_TAG, Shape, WIRE_TAG};
use crate::layout::{CELL_RADIUS, Layout, PORT_RADIUS, SURFACE_RADIUS, WIRE_HALF_WIDTH_QUARTERS};

/// Surface (body slot 0), cells (slots in name order), wires (link slots in
/// `print_body` order), then ports. Every generation is 1.
pub fn shapes_of_layout(layout: &Layout) -> Vec<Shape> {
    let m = layout.surface;
    let mut shapes = vec![Shape {
        geom: Geom::RoundRect {
            x: m.x,
            y: m.y,
            w: m.w,
            h: m.h,
            r: SURFACE_RADIUS,
        },
        id: [1, 0, 0, 1],
    }];
    for (slot, c) in (1u32..).zip(&layout.cells) {
        shapes.push(Shape {
            geom: Geom::RoundRect {
                x: c.rect.x,
                y: c.rect.y,
                w: c.rect.w,
                h: c.rect.h,
                r: CELL_RADIUS,
            },
            id: [1, slot, 0, 1],
        });
    }
    for (slot, w) in (0u32..).zip(&layout.wires) {
        shapes.push(Shape {
            geom: Geom::Capsule {
                a: w.from,
                b: w.to,
                half_quarters: WIRE_HALF_WIDTH_QUARTERS,
            },
            id: [1, 0, WIRE_TAG | slot, 1],
        });
    }
    for p in &layout.ports {
        let cell = (1u32..)
            .zip(&layout.cells)
            .find(|(_, c)| c.instance == p.address.instance)
            .map_or(0, |(slot, _)| slot);
        shapes.push(Shape {
            geom: Geom::Circle {
                x: p.x,
                y: p.y,
                r: PORT_RADIUS,
            },
            id: [1, cell, PORT_TAG | p.address.port, 1],
        });
    }
    shapes
}
