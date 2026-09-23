//! Finish a nested fire return.

use joinn_frame::{Value, Verdict};
use std::collections::BTreeMap;

use crate::report::StepReport;
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn finish_fire(
        &mut self,
        name: &str,
        outs: BTreeMap<u32, Value>,
        direction: Option<u32>,
    ) -> Verdict<StepReport> {
        let depth = self.stack.len().saturating_sub(1); // allow(vocab): rust usize saturating_sub, not a turn identifier
        self.consume_and_emit(depth, name, &[], &outs);
        if let Some(prev) = self.last_ports.get_mut(name) {
            for (k, v) in &outs {
                prev.insert(*k, v.clone());
            }
        } else {
            self.last_ports.insert(name.into(), outs.clone());
        }
        self.enqueue_outs(depth, name, &outs);
        self.advance_grants(depth, name);
        Verdict::Ok(StepReport {
            step: self.step,
            fired: Some(name.into()),
            direction,
            delivered: Vec::new(),
            checks: vec!["nested return".into()],
            depth: self.depth(),
        })
    }
}
