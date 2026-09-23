//! Advance grant holders after a fire.

use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn advance_grants(&mut self, depth: usize, name: &str) {
        if depth != 0 {
            return;
        }
        let mut next_grants = Vec::new();
        if let Some(act) = self.stack.get(depth) {
            for (cap, holder) in &act.grant_holder {
                if holder == name {
                    if let Some(order) = act.body.coding.grants.get(cap) {
                        if let Some(idx) = order.iter().position(|i| i == name) {
                            if let Some(nxt) = order.get(idx + 1) {
                                next_grants.push((cap.clone(), nxt.clone()));
                            }
                        }
                    }
                }
            }
        }
        if let Some(act) = self.stack.get_mut(depth) {
            for (cap, nxt) in next_grants {
                act.grant_holder.insert(cap, nxt);
            }
        }
    }
}
