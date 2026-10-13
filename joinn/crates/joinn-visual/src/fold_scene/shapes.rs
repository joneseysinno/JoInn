//! The fold's live rows as shapes.

use super::FoldScene;
use crate::dna_draw::scene_shapes;
use crate::pick::Shape;

impl FoldScene {
    /// The surface, then every stroke in draw order.
    pub fn shapes(&self) -> Vec<Shape> {
        scene_shapes(&self.tables)
    }
}
