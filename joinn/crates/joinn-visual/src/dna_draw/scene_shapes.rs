//! A picture's live rows as shapes, in the GPU's draw order.

use crate::pick::{Geom, Shape};
use crate::tables::{LIVE, Tables, shapes_of_tables};

/// The surface, then every live stroke by slot.
pub(crate) fn scene_shapes(tables: &Tables) -> Vec<Shape> {
    let mut shapes = shapes_of_tables(tables);
    shapes.extend(
        tables
            .stroke
            .iter()
            .filter(|s| s.flags & LIVE != 0)
            .map(|s| Shape {
                geom: Geom::Stroke {
                    a: (s.x0.into(), s.y0.into()),
                    b: (s.x1.into(), s.y1.into()),
                    half: s.half_width.into(),
                },
                id: [s.owner[0], s.owner[1], s.owner[2], s.generation],
            }),
    );
    shapes
}
