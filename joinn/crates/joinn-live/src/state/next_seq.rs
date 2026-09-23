//! Next mail sequence number.

use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn next_seq(&mut self) -> u64 {
        let s = self.seq;
        self.seq += 1;
        s
    }
}
