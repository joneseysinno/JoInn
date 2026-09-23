//! The three Phase 1 frames.

pub mod int;
pub mod rat;
pub mod text;

pub use int::IntFrame;
pub use rat::RatFrame;
pub use text::TextFrame;

/// Default generator size for conformance and the gate.
pub const DEFAULT_SIZE: u8 = 16;
