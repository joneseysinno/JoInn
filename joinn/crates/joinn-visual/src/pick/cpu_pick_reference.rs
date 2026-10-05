//! The brute-force picker through a Phase 6 camera.

use joinn_frame::Verdict;

use super::cpu_pick_reference_at::cpu_pick_reference_at;
use super::{PickImage, Shape};
use crate::camera::{Camera, FitCamera};

/// `cpu_pick_reference_at` through the exact camera the fit camera converts to.
pub fn cpu_pick_reference(shapes: &[Shape], camera: &FitCamera) -> Verdict<PickImage> {
    match Camera::from_fit(camera) {
        Verdict::Ok(c) => cpu_pick_reference_at(shapes, &c),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}
