//! Nested activation depth.

use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn depth(&self) -> u32 {
        self.stack.len().saturating_sub(1) as u32 // allow(vocab): rust usize saturating_sub, not a turn identifier
    }
}
