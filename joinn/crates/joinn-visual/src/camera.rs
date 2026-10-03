//! The camera. `FitCamera` is Phase 6's whole-number camera, kept for gates 6
//! and 7. `Camera` is exact at any zoom: a level and a step, a dyadic focus in
//! the anchor chart, and a whole pixel it is pinned to. A camera change writes
//! the tick uniform and no table row (V121, V142): tables hold layout units,
//! never pixels.

mod fit;
mod frame;
mod from_fit;
mod pan;
mod pixel_of;
mod print_camera;
mod print_zoom;
mod refocus;
mod resize;
mod zoom_about;
mod zoom_from_whole_k;
mod zoom_k;
mod zoom_new;
mod zoom_notch;
mod zoom_whole_k;

pub use fit::fit;
pub use print_camera::print_camera;
pub use print_zoom::print_zoom;

/// `k` pixels per layout unit and an integer pixel origin. A layout point
/// `(u, v)` is the pixel point `(ox + k·u, oy + k·v)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FitCamera {
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

/// The least zoom level.
pub const LEVEL_MIN: i32 = -4;
/// The greatest zoom level.
pub const LEVEL_MAX: i32 = 9;
/// Fine steps between one level and the next.
pub const STEPS: u32 = 256;
/// Fine steps in one wheel notch. Eight notches double `k`.
pub const NOTCH: u32 = 32;
/// Focus units in one layout unit: the focus is in 2^-16 layout units.
pub const FOCUS_UNIT: i64 = 1 << 16;
/// Pixel-fraction units in one pixel: `pixel_of` answers in 2^-32 px.
pub const PIXEL_UNIT: i128 = 1 << 32;

/// A zoom: `k = 2^level · (256 + step) / 256` pixels per layout unit.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Zoom {
    /// Whole doublings, `LEVEL_MIN ..= LEVEL_MAX`.
    pub level: i32,
    /// Fine step toward the next doubling, `0 .. STEPS`.
    pub step: u32,
}

/// A chart: a slot in its universe layout's chart list. Slot 0 is the root
/// (the universe chart, or a single-body scene's body chart).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ChartId(pub u32);

/// The exact camera. The pixel of anchor-chart point `p` is
/// `pin + k·(p − focus)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Camera {
    /// The zoom.
    pub zoom: Zoom,
    /// The chart `focus` is a point of.
    pub anchor: ChartId,
    /// A point of the anchor chart, in 2^-16 layout units.
    pub focus: (i64, i64),
    /// The whole pixel the focus sits on.
    pub pin: (i64, i64),
    /// Viewport width in pixels.
    pub width: u32,
    /// Viewport height in pixels.
    pub height: u32,
}
