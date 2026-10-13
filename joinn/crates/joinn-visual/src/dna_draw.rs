//! What the strand view and the fold view share (plan 8 §2.10): a piece is
//! one capsule in sixteenths of the scene's one body chart; the pieces become
//! stroke rows over body 0 and chart 0. Neither view looks a codon up: they
//! draw what the reader and the tower give them.

mod fit_body;
mod label;
mod meaning_style;
mod owner;
mod scene_shapes;
mod write_pieces;

pub(crate) use fit_body::fit_body;
pub(crate) use label::label;
pub(crate) use meaning_style::meaning_style;
pub(crate) use owner::owner;
pub(crate) use scene_shapes::scene_shapes;
pub(crate) use write_pieces::write_pieces;

/// One capsule: from `a` to `b` with half-width `half`, all in sixteenths.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Piece {
    /// One end.
    pub(crate) a: (i64, i64),
    /// The other end.
    pub(crate) b: (i64, i64),
    /// Half-width.
    pub(crate) half: i64,
    /// Style id.
    pub(crate) style: u32,
    /// The ID's first three channels.
    pub(crate) owner: [u32; 3],
}
