//! Fit the whole fold to a viewport.

use super::FoldScene;
use crate::camera::FitCamera;
use crate::dna_draw::fit_body;

impl FoldScene {
    /// Phase 6's `fit` on the picture's body; every floor has the same body.
    pub fn fit(&self, width: u32, height: u32) -> FitCamera {
        fit_body(&self.tables, width, height)
    }
}
