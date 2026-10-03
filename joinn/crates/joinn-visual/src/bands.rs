//! Bands: how a thing is drawn follows its projected size `s = k·max(w, h)`
//! in pixels. Every band decision is an integer test (V141).

mod fade_window;
mod owner_band;

pub use fade_window::fade_window;
pub use owner_band::owner_band;

/// Band thresholds in pixels: dot below 4, glyph to 32, summary to 240, full.
pub const THRESHOLDS: [i64; 3] = [4, 32, 240];

/// How a body is drawn.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Band {
    /// A disc of radius 2 px at the surface's centre.
    Dot,
    /// The surface.
    Glyph,
    /// The surface, the cells and the surface ports.
    Summary,
    /// Everything, with text.
    Full,
}
