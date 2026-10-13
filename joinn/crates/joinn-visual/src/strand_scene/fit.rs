//! Fit the whole strand to a viewport.

use super::StrandScene;
use crate::camera::FitCamera;
use crate::dna_draw::fit_body;

impl StrandScene {
    /// Phase 6's `fit` on the picture's body.
    pub fn fit(&self, width: u32, height: u32) -> FitCamera {
        fit_body(&self.tables, width, height)
    }
}
