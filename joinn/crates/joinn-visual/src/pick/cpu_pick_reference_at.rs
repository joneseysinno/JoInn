//! The brute-force picker: every shape, every pixel. It exists to be agreed with.

use joinn_frame::Verdict;

use super::cpu_pick_sample::cpu_pick_sample;
use super::{PickImage, Shape};
use crate::camera::Camera;

/// `cpu_pick_sample` at every pixel of the viewport, row by row.
pub fn cpu_pick_reference_at(shapes: &[Shape], camera: &Camera) -> Verdict<PickImage> {
    let every: Vec<(u32, u32)> = (0..camera.height)
        .flat_map(|y| (0..camera.width).map(move |x| (x, y)))
        .collect();
    cpu_pick_sample(shapes, camera, &every).map(|pixels| PickImage {
        width: camera.width,
        height: camera.height,
        pixels,
    })
}
