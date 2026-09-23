//! How many values a drive bound admits in `frame`.

use joinn_frame::{FrameId, FrameRef};
use std::num::NonZeroU32;

/// How many values a drive bound admits in `frame`.
pub fn drive_admits(frame: FrameRef, bound: NonZeroU32) -> u32 {
    let b = bound.get();
    match frame.id {
        FrameId::Int | FrameId::Rat => b.saturating_mul(2).saturating_add(1),
        FrameId::Text => b.saturating_add(1),
    }
}
