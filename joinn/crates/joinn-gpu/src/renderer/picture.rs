//! Draw offscreen and read back both targets.

use joinn_frame::Verdict;
use joinn_visual::Camera;

use super::read_target::read_target;
use super::{Picture, Renderer};
use crate::gpu::Gpu;
use crate::refuse::refuse;

impl Renderer {
    /// One `frame` at the camera's viewport, then the color and ID targets read
    /// back row by row.
    pub fn picture(&mut self, gpu: &Gpu, camera: &Camera) -> Verdict<Picture> {
        if let Verdict::Refused(r) = self.frame(gpu, camera) {
            return Verdict::Refused(r);
        }
        let (Some(color), Some(ids)) = (self.color.as_ref(), self.ids.as_ref()) else {
            return refuse("picture: the frame left no targets; acceptance is a drawn frame");
        };
        let color = match read_target(gpu, color, 4) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Verdict::Refused(r),
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
