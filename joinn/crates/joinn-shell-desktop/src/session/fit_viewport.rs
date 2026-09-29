//! A resize writes a camera and no table row.

use joinn_visual::fit;

use super::Desktop;

impl Desktop {
    /// Fit the surface to `width` × `height`. Tables are unchanged.
    pub fn fit_viewport(&mut self, width: u32, height: u32) {
        self.camera = fit(self.scene.layout(), width, height);
    }
}
