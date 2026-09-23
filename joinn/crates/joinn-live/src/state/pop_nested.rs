//! Pop nested activation and finish fire.

use joinn_dna::Direction;
use joinn_frame::{CheckId, Refusal, Verdict};
use std::collections::BTreeMap;

use crate::report::StepReport;
use crate::slot::slot_value;
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn pop_nested(&mut self) -> Verdict<StepReport> {
        let nested = match self.stack.pop() {
            Some(a) => a,
            None => {
                return Verdict::Refused(Refusal::structural(CheckId::Other, "pop of empty stack"));
            }
        };
        let parent_name = nested.return_to.clone();
        let mut outs = BTreeMap::new();
        if let Some(bname) = &nested.boundary {
            if let Some(inst) = nested.instances.get(bname) {
                for p in &inst.cell.coding.contract.ports {
                    if p.direction == Direction::Out {
                        if let Some(v) = slot_value(inst.slots.get(&p.position)) {
                            outs.insert(p.position, v);
                        }
                    }
                }
            }
        }
        let Some(pname) = parent_name else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                "nested activation missing return",
            ));
        };
        self.finish_fire(&pname, outs, None)
    }
}
