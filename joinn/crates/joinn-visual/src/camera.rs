//! The camera (Chart, Phase 6 form). A camera change writes the tick uniform and
//! no table row (V121): tables hold layout units, never pixels.

mod fit;
mod print_camera;

pub use fit::fit;
pub use print_camera::print_camera;

/// `k` pixels per layout unit and an integer pixel origin. A layout point
/// `(u, v)` is the pixel point `(ox + k·u, oy + k·v)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Camera {
    /// Pixels per layout unit. A multiple of 4, so a quarter unit is whole pixels.
    pub k: i64,
    /// Pixel x of layout x 0.
    pub ox: i64,
    /// Pixel y of layout y 0.
    pub oy: i64,
    /// Viewport width in pixels.
    pub width: u32,
    /// Viewport height in pixels.
    pub height: u32,
}

/// The four standard viewports. Every calculator check runs at all four.
pub const STANDARD_VIEWPORTS: [(u32, u32); 4] =
    [(640, 360), (1000, 777), (1280, 720), (1920, 1080)];
