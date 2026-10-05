//! JoInn's stroke font (plan 7.2 §2.8). Every glyph is a set of straight
//! strokes on a grid 6 wide and 8 tall (baseline at 8, descender to 10), each
//! drawn as a capsule. Round letters are polygons. Text is placed in sixteenths
//! of a layout unit inside its chart; nothing here is a float.

mod cell_label;
mod glyph;
mod port_value;
mod text_strokes;
mod title;
mod value_text;

pub use cell_label::cell_label;
pub use glyph::{STROKE_SET, glyph};
pub use port_value::port_value;
pub use text_strokes::text_strokes;
pub use title::title;
pub use value_text::value_text;

/// Grid width of a glyph box.
pub const GRID_W: i64 = 6;
/// Grid height from cap top to baseline.
pub const GRID_H: i64 = 8;
/// Grid depth of the descender, from the cap top.
pub const GRID_DESCENT: i64 = 10;
/// Grid units from one glyph's origin to the next.
pub const ADVANCE: i64 = 8;
/// Glyphs an out-port value prints before it is cut to three and `…`.
pub const VALUE_GLYPHS: usize = 4;

/// One stroke of a glyph, in grid units: from `(x0, y0)` to `(x1, y1)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GlyphStroke {
    /// Start x.
    pub x0: i64,
    /// Start y, down from the cap top.
    pub y0: i64,
    /// End x.
    pub x1: i64,
    /// End y.
    pub y1: i64,
}

/// One placed stroke, in sixteenths of a layout unit inside its chart.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TextStroke {
    /// Start x.
    pub x0: i64,
    /// Start y.
    pub y0: i64,
    /// End x.
    pub x1: i64,
    /// End y.
    pub y1: i64,
    /// Capsule half-width.
    pub half_width: i64,
}

/// How big a run of text is: its grid unit and stroke half-width, both in
/// sixteenths.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TextSize {
    /// Sixteenths per grid unit.
    pub unit: i64,
    /// Stroke half-width in sixteenths.
    pub half_width: i64,
}

/// A cell label: cap height 1 layout unit.
pub const LABEL_SIZE: TextSize = TextSize {
    unit: 2,
    half_width: 1,
};
/// An out-port value: cap height 2.
pub const VALUE_SIZE: TextSize = TextSize {
    unit: 4,
    half_width: 2,
};
/// A system title: cap height 6.
pub const SYSTEM_TITLE_SIZE: TextSize = TextSize {
    unit: 12,
    half_width: 6,
};
/// A galaxy title: cap height 16.
pub const GALAXY_TITLE_SIZE: TextSize = TextSize {
    unit: 32,
    half_width: 16,
};

/// Which frame a title names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TitleOf {
    /// A system: at its chart's `(8, 4)`.
    System,
    /// A galaxy: at its chart's `(16, 8)`.
    Galaxy,
}
