//! One offscreen frame, finished on the GPU.

use joinn_frame::Verdict;
use joinn_visual::FitCamera;

use super::target::target;
use super::{OFFSCREEN_FORMAT, Renderer};
use crate::gpu::{Gpu, wait};
use crate::refuse::refuse;

impl Renderer {
    /// Draws the tables as last uploaded into this renderer's own color target
    /// at the camera's viewport, and waits until the GPU has finished.
    pub fn frame(&mut self, gpu: &Gpu, camera: &FitCamera) -> Verdict<()> {
        if self.format != OFFSCREEN_FORMAT {
            return refuse(format!(
                "frame: this renderer writes {:?}; acceptance is a renderer built for {OFFSCREEN_FORMAT:?}",
                self.format
            ));
        }
        let (width, height) = (camera.width, camera.height);
        let resize = self
            .color
            .as_ref()
            .is_none_or(|t| t.width() != width || t.height() != height);
        if resize {
            self.color = Some(target(gpu.device(), OFFSCREEN_FORMAT, width, height));
        }
        let Some(color) = self.color.as_ref() else {
            return refuse("frame: no color target; acceptance is a renderer that made one");
        };
        let view = color.create_view(&wgpu::TextureViewDescriptor::default());
        if let Verdict::Refused(r) = self.draw(gpu, camera, &view) {
            return Verdict::Refused(r);
        }
        wait(gpu)
    }
}
