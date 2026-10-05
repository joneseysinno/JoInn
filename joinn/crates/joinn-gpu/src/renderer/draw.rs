//! Draw through a Phase 6 camera.

use joinn_frame::Verdict;
use joinn_visual::{Camera, FitCamera};

use super::Renderer;
use crate::gpu::Gpu;

impl Renderer {
    /// `draw_at` through the exact camera the fit camera converts to.
    pub fn draw(
        &mut self,
        gpu: &Gpu,
        camera: &FitCamera,
        color: &wgpu::TextureView,
    ) -> Verdict<()> {
        match Camera::from_fit(camera) {
            Verdict::Ok(c) => self.draw_at(gpu, &c, color),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}
