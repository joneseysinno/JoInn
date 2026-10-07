//! One growth step, applied to the tables as a delta.

use joinn_frame::{FrameRegistry, Value, Verdict};
use joinn_link::{grow_step, respond};

use super::SystemScene;
use crate::system_layout::layout_system;
use crate::tables::Delta;

impl SystemScene {
    /// The body accepts `input` (or refuses it, and nothing changes), the
    /// engine gives the new count, and the rows the step changes are written.
    pub fn apply_growth(&mut self, input: &Value) -> Verdict<Delta> {
        let grown = match grow_step(&self.grown, input) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let count = match respond(&grown, &self.cells, &FrameRegistry::phase1()) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let layout = match layout_system(
            grown.system(),
            &self.contacts,
            &self.cells,
            &grown,
            self.waiting,
        ) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut next = self.clone();
        next.grown = grown;
        next.count = count.clone();
        let delta = match next.write_rows(layout, &count) {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        *self = next;
        Verdict::Ok(delta)
    }
}
