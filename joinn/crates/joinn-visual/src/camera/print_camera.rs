//! One camera line: `camera <w>x<h>: k <k>, origin <ox> <oy>`.

use super::Camera;

/// One camera line: `camera <w>x<h>: k <k>, origin <ox> <oy>`.
pub fn print_camera(camera: &Camera) -> String {
    format!(
        "camera {}x{}: k {}, origin {} {}",
        camera.width, camera.height, camera.k, camera.ox, camera.oy
    )
}
