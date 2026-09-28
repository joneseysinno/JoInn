//! The tick uniform: the camera, and nothing else.

use joinn_frame::Verdict;
use joinn_visual::Camera;

use crate::refuse::refuse;

/// `k ox oy width height`, then three zeros. A camera that doesn't fit 32 bits
/// is refused.
pub(super) fn tick_bytes(camera: &Camera) -> Verdict<[i32; 8]> {
    let fields = (
        i32::try_from(camera.k),
        i32::try_from(camera.ox),
        i32::try_from(camera.oy),
        i32::try_from(camera.width),
        i32::try_from(camera.height),
    );
    match fields {
        (Ok(k), Ok(ox), Ok(oy), Ok(w), Ok(h)) => Verdict::Ok([k, ox, oy, w, h, 0, 0, 0]),
        _ => refuse(format!(
            "tick: camera {camera:?} does not fit the tick uniform's 32-bit fields; acceptance is a camera within 32 bits"
        )),
    }
}
