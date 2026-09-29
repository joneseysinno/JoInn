//! Hand the pending GPU confirmation to the next tick.

use super::{Confirm, Desktop};

impl Desktop {
    /// `None` after an edge click: nothing to confirm.
    pub fn take_confirm(&mut self) -> Option<Confirm> {
        self.confirm.take()
    }
}
