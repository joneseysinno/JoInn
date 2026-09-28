//! Draw offscreen and read back both targets.

use joinn_frame::Verdict;
use joinn_visual::Camera;

use super::read_target::read_target;
use super::target::target;
use super::{OFFSCREEN_FORMAT, Picture, Renderer};
use crate::gpu::Gpu;
use crate::refuse::refuse;

impl Renderer {
    /// Draws the tables as last uploaded at the camera's viewport into a fresh
    /// color target, then reads back the color and ID targets.
    pub fn picture(&mut self, gpu: &Gpu, camera: &Camera) -> Verdict<Picture> {
        if self.format != OFFSCREEN_FORMAT {
            return refuse(format!(
                "picture: this renderer writes {:?}; acceptance is a renderer built for {OFFSCREEN_FORMAT:?}",
                self.format
            ));
        }
        let color = target(gpu.device(), OFFSCREEN_FORMAT, camera.width, camera.height);
        let view = color.create_view(&wgpu::TextureViewDescriptor::default());
        if let Verdict::Refused(r) = self.draw(gpu, camera, &view) {
            return Verdict::Refused(r);
        }
        let color = match read_target(gpu, &color, 4) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let Some(ids) = self.ids.as_ref() else {
            return refuse("picture: the draw left no ID target; acceptance is a drawn frame");
        };
        read_target(gpu, ids, 16).map(|raw| Picture {
            color,
            ids: raw
                .chunks_exact(16)
                .map(|t| {
                    let word = |i: usize| u32::from_le_bytes([t[i], t[i + 1], t[i + 2], t[i + 3]]);
                    [word(0), word(4), word(8), word(12)]
                })
                .collect(),
        })
    }
}
