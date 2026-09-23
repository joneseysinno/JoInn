//! Push a DNA body as nested activation.

use joinn_frame::{CheckId, Hash, Refusal, Subject, Value, Verdict};
use std::collections::BTreeMap;

use crate::activation::{build_activation, fan_boundary};
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn push_dna(
        &mut self,
        depth: usize,
        name: &str,
        body_hash: Hash,
        inputs: BTreeMap<u32, Value>,
        consume: Vec<u32>,
    ) -> Verdict<Option<String>> {
        let Some(body) = self.bodies.get(&body_hash).cloned() else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!("{name} Dna body {} is not loaded", body_hash.to_hex()),
            ));
        };
        if !body.coding.grants.is_empty() {
            return Verdict::Refused(Refusal {
                check: CheckId::Grant,
                subject: Subject::Other(name.into()),
                reason: format!(
                    "nested activation holds no grants (found {})",
                    body.coding
                        .grants
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                counterexample: None,
                seed: self.seed,
            });
        }
        self.consume_and_emit(depth, name, &consume, &BTreeMap::new());
        let mut nested = match build_activation(
            &body,
            &self.cells,
            Some(name.to_string()),
            name,
            true,
            self.seed,
        ) {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if let Some(bname) = nested.boundary.clone() {
            fan_boundary(&mut nested, &bname, &inputs, &mut self.seq);
        }
        self.stack.push(nested);
        Verdict::Ok(None)
    }
}
