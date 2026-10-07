//! The system's live rows as shapes, in the GPU's draw order.

use super::SystemScene;
use crate::pick::{Geom, Shape};
use crate::tables::{LIVE, STYLE_FORCE, StrokeRow, shapes_of_tables};

impl SystemScene {
    /// The lasso's strokes (under every body, cell and port), then the
    /// tables' shapes, then the text strokes on top (plan 7.4 §2.8's order).
    pub fn shapes(&self) -> Vec<Shape> {
        let stroke = |s: &StrokeRow| Shape {
            geom: Geom::Stroke {
                a: (s.x0.into(), s.y0.into()),
                b: (s.x1.into(), s.y1.into()),
                half: s.half_width.into(),
            },
            id: [s.owner[0], s.owner[1], s.owner[2], s.generation],
        };
        let live = self.tables.stroke.iter().filter(|s| s.flags & LIVE != 0);
        let (lasso, text): (Vec<&StrokeRow>, Vec<&StrokeRow>) =
            live.partition(|s| s.style == STYLE_FORCE);
        let mut shapes: Vec<Shape> = lasso.into_iter().map(stroke).collect();
        shapes.extend(shapes_of_tables(&self.tables));
        shapes.extend(text.into_iter().map(stroke));
        shapes
    }
}
