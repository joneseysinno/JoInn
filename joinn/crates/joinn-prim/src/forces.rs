//! The force register: how a frame responds to a force (Part V C10). It is frame
//! vocabulary for forces, one short table keyed by (force, frame), and a combine
//! is admitted only beside its separate (C23).

mod check_register;
mod order_free;
mod register;
mod response;

pub use check_register::check_register;
pub use order_free::order_free;
pub use register::register;
pub use response::response;

use joinn_frame::{FrameId, FrameRef, Hash};
use std::num::NonZeroU32;

/// Pairs and triples the register is admitted at (rule 20).
pub const REGISTER_BOUND: NonZeroU32 = match NonZeroU32::new(64) {
    Some(n) => n,
    None => NonZeroU32::MIN,
};

/// The seed the register is admitted at.
pub const REGISTER_SEED: u64 = 7;

/// A force. Closed: Phase 7 writes combine only.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ForceKind {
    /// Cells brought together; the response appears beside them.
    Combine,
}

impl ForceKind {
    /// The word a `.contact` file writes.
    pub fn word(self) -> &'static str {
        match self {
            ForceKind::Combine => "combine",
        }
    }
}

/// How `frame` responds to `force`, and the opposite that separates it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RegisterRow {
    /// The force.
    pub force: ForceKind,
    /// The frame acted on: name and version, `("ℤ", 1)`.
    pub frame: (&'static str, u16),
    /// The coding region that says how the frame responds.
    pub response: Hash,
    /// Its opposite: a turn of `response`.
    pub separate: Hash,
}

impl RegisterRow {
    /// The row's frame, when its name is a frame JoInn knows.
    pub fn frame_ref(&self) -> Option<FrameRef> {
        FrameId::parse(self.frame.0).map(|id| FrameRef::new(id, u32::from(self.frame.1)))
    }
}
