//! `cargo xtask zoom`: the grove's cut at plan 7.2 §2.12's views, and its
//! touches at two of them.

mod cut_line;
mod fade_views;
mod zoom_cmd;
mod zoom_measure;
mod zoom_script;
mod zoom_text;
mod zoom_views;

pub(crate) use cut_line::cut_line;
pub(crate) use fade_views::fade_views;
pub(crate) use zoom_cmd::zoom;
pub(crate) use zoom_measure::zoom_measure;
pub(crate) use zoom_script::zoom_script;
pub(crate) use zoom_text::zoom_text;
pub(crate) use zoom_views::zoom_views;

use joinn_visual::FOCUS_UNIT;

/// The viewport every view is taken at.
pub(crate) const VIEWPORT: (u32, u32) = (1920, 1080);
/// The centre of the `s` of `b0000`'s `sum` label, in universe focus units:
/// `(113 3/8, 117 1/4)` (§2.8).
pub(crate) const S_CENTRE: (i64, i64) = (
    113 * FOCUS_UNIT + 3 * FOCUS_UNIT / 8,
    117 * FOCUS_UNIT + FOCUS_UNIT / 4,
);

/// §2.12's block, byte for byte: what `cargo xtask zoom` prints.
pub(crate) const ZOOM_BLOCK: &str = "\
zoom 1920x1080 frame level -2 step 95 (k 351/1024), anchor universe: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 0, glyph 3072, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -4 step 0 (k 1/16), anchor b0000: galaxies 8 open, 0 nodes · systems 0 open, 128 nodes · bodies dot 0, glyph 0, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -3 step 0 (k 1/8), anchor b0000: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 1891, glyph 1181, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -2 step 0 (k 1/4), anchor b0000: galaxies 6 open, 0 nodes · systems 96 open, 0 nodes · bodies dot 0, glyph 2240, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -1 step 0 (k 1/2), anchor b0000: galaxies 4 open, 0 nodes · systems 36 open, 0 nodes · bodies dot 0, glyph 864, summary 0, full 0 · fading 0
zoom 1920x1080 at s level 0 step 0 (k 1), anchor b0000: galaxies 1 open, 0 nodes · systems 16 open, 0 nodes · bodies dot 0, glyph 153, summary 113, full 0 · fading 0
zoom 1920x1080 at s level 1 step 0 (k 2), anchor b0000: galaxies 1 open, 0 nodes · systems 4 open, 0 nodes · bodies dot 0, glyph 0, summary 80, full 0 · fading 0
zoom 1920x1080 at s level 2 step 0 (k 4), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 24, full 0 · fading 0
zoom 1920x1080 at s level 3 step 0 (k 8), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 4, full 8 · fading 0
zoom 1920x1080 at s level 4 step 0 (k 16), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 4 · fading 0
zoom 1920x1080 at s level 5 step 0 (k 32), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 2 · fading 0
zoom 1920x1080 at s level 6 step 0 (k 64), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 7 step 0 (k 128), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 8 step 0 (k 256), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 9 step 0 (k 512), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
touches level -4 step 0: 137 links, 264 touches, each node at most once per link
touches frame: 137 links, 1182 touches, each node at most once per link
zoom: 15 views, cut counts printed; touches 264 and 1182
";
