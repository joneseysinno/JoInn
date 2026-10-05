//! Draw offscreen through a Phase 6 camera and read back both targets.

use joinn_frame::Verdict;
use joinn_visual::{Camera, FitCamera};

use super::{Picture, Renderer};
use crate::gpu::Gpu;

impl Renderer {
    /// `picture_at` through the exact camera the fit camera converts to.
    pub fn picture(&mut self, gpu: &Gpu, camera: &FitCamera) -> Verdict<Picture> {
        match Camera::from_fit(camera) {
            Verdict::Ok(c) => self.picture_at(gpu, &c),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}
