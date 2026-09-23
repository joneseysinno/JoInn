//! Consume inputs and write outputs on an instance.

use joinn_frame::Value;
use std::collections::BTreeMap;

use crate::slot::{SlotWrite, apply_slot};
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn consume_and_emit(
        &mut self,
        depth: usize,
        name: &str,
        consume: &[u32],
        outs: &BTreeMap<u32, Value>,
    ) {
        let seed = self.seed;
        if let Some(act) = self.stack.get_mut(depth) {
            if let Some(inst) = act.instances.get_mut(name) {
                let policy = inst.cell.coding.contract.join_policy;
                for pos in consume {
                    let _ = apply_slot(inst, *pos, SlotWrite::Consume, policy, name, seed);
                }
                for (pos, val) in outs {
                    // Port 0 is in and out on eq/choose. Writing the output
                    // back onto that in-slot makes the next mail re-fire
                    // with the leftover as a fresh condition.
                    if consume.contains(pos) {
                        continue;
                    }
                    let _ =
                        apply_slot(inst, *pos, SlotWrite::Fill(val.clone()), policy, name, seed);
                }
            }
        }
    }
}
