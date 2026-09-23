//! Phase 1 frame set.

use super::FrameRegistry;
use crate::frames::{IntFrame, RatFrame, TextFrame};
use std::sync::Arc;

impl FrameRegistry {
    /// The three Phase 1 frames.
    pub fn phase1() -> Self {
        let mut reg = Self::new();
        reg.register(Arc::new(TextFrame::new()));
        reg.register(Arc::new(IntFrame::new()));
        reg.register(Arc::new(RatFrame::new()));
        reg
    }
}
