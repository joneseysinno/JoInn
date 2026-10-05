//! One offscreen frame through a Phase 6 camera.

use joinn_frame::Verdict;
use joinn_visual::{Camera, FitCamera};

use super::Renderer;
use crate::gpu::Gpu;

impl Renderer {
    /// `frame_at` through the exact camera the fit camera converts to.
    pub fn frame(&mut self, gpu: &Gpu, camera: &FitCamera) -> Verdict<()> {
        match Camera::from_fit(camera) {
            Verdict::Ok(c) => self.frame_at(gpu, &c),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}
