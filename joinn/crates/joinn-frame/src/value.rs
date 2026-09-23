//! Frame-tagged canonical values. Constructible only through a frame.

mod product;
mod unproduct;

use crate::frame::FrameRef;
use crate::frames::{int, rat, text};
use crate::term::Term;

/// Always frame-tagged, always canonical.
///
/// There is no public constructor. The only way to obtain a `Value` is
/// `Frame::canonicalize` (or `Frame::parse` / `Frame::generate`, which call it).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Value {
    pub(in crate::value) frame: FrameRef,
    pub(in crate::value) term: Term,
}

impl Value {
    /// Crate-internal constructor used only after a frame has canonicalized.
    pub(crate) fn from_canonical(frame: FrameRef, term: Term) -> Self {
        Self { frame, term }
    }

    /// The value's frame.
    pub fn frame(&self) -> &FrameRef {
        &self.frame
    }

    /// The canonical term.
    pub fn term(&self) -> &Term {
        &self.term
    }

    /// Canonical literal text of the term, without the frame prefix.
    pub fn print_term(&self) -> String {
        match self.frame.id {
            crate::frame::FrameId::Text => text::print_term(&self.term),
            crate::frame::FrameId::Int => int::print_term(&self.term),
            crate::frame::FrameId::Rat => rat::print_term(&self.term),
        }
    }

    /// Canonical literal including the frame reference: `ℤ 1 5`.
    pub fn print_literal(&self) -> String {
        format!("{} {}", self.frame, self.print_term())
    }
}
