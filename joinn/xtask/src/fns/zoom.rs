//! `cargo xtask zoom`: the grove's cut at plan 7.2 §2.12's views, and its
//! touches at two of them.

mod cut_line;
mod zoom_cmd;
mod zoom_text;
mod zoom_views;

pub(crate) use cut_line::cut_line;
pub(crate) use zoom_cmd::zoom;
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
