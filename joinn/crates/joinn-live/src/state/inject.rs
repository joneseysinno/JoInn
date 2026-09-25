//! Inject a value at an instance in-port.

use joinn_frame::{CheckId, Refusal, Subject, Value, Verdict};

use crate::activation::Mail;
use crate::slot::{SlotWrite, apply_slot};
use crate::state::BodyState;

impl BodyState {
    pub fn inject(
        &mut self,
        instance: &str,
        port: u32,
        value: Value,
        grant_epoch: u64,
    ) -> Verdict<()> {
        if self.stack.len() > 1 {
            return Verdict::Refused(Refusal {
                check: CheckId::Grant,
                subject: Subject::Other(instance.into()),
                reason: format!("nested activation cannot read capability at {instance}"),
                counterexample: None,
                seed: self.seed,
            });
        }
        let Some(root) = self.stack.first() else {
            return Verdict::Refused(Refusal::structural(CheckId::Other, "empty stack"));
        };
        if !root.instances.contains_key(instance) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Grant,
                format!("inject: unknown instance {instance}"),
            ));
        }
        let granted = root
            .body
            .coding
            .grants
            .iter()
            .any(|(_, insts)| insts.iter().any(|i| i == instance));
        if !granted && !root.body.coding.grants.is_empty() {
            let cap = root
                .body
                .coding
                .grants
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| "grant".into());
            return Verdict::Refused(Refusal {
                check: CheckId::Grant,
                subject: Subject::Other(instance.into()),
                reason: format!("ungranted read at {instance} capability {cap}"),
                counterexample: None,
                seed: self.seed,
            });
        }
        // After a membrane refusal the in-port may still hold the rejected value;
        // clear it so a recovery inject is not a join refuse. describe_refusal
        // must run before this inject (CLI probe order).
        if self.last_refusal.is_some() {
            let seed = self.seed;
            if let Some(root) = self.stack.first_mut() {
                if let Some(inst) = root.instances.get_mut(instance) {
                    let policy = inst.cell.coding.contract.join_policy;
                    let _ = apply_slot(inst, port, SlotWrite::Consume, policy, instance, seed);
                }
            }
        }
        let seq = self.next_seq();
        self.enqueue(
            0,
            Mail {
                dest: instance.into(),
                port,
                value,
                grant_epoch,
                wire_position: 0,
                message_sequence: seq,
            },
        );
        Verdict::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{int, natives, sum_cli_body, text};
    use joinn_dna::{AlleleBody, hash, parse_body, sum_cell};
    use joinn_frame::FrameRegistry;
    use std::collections::BTreeMap;

    #[test]
    fn ungranted_read_refuses() {
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let src = format!(
            "body {{ codex 1 genome {{ cell:{} as sum }} grants {{ cli cli_missing }} wires {{ }} budget {{ steps 10 }} lineage none }}\n",
            sh.to_hex()
        );
        let body = match parse_body(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut cells = BTreeMap::new();
        cells.insert(sh, sum);
        let mut state = match BodyState::new(body, cells, natives(), 7) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match state.inject("sum", 0, int(1), 0) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("ungranted"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("ungranted inject must refuse"),
        }
    }

    #[test]
    fn nested_capability_read_refuses() {
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let src = format!(
            "body {{ codex 1 genome {{ cell:{} as self prim:eq as e }} grants {{ cli self }} wires {{ }} budget {{ steps 10 }} lineage none }}\n",
            sh.to_hex()
        );
        let body = match parse_body(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let bh = hash(&body.coding);
        let mut cells = BTreeMap::new();
        let mut sum_dna = sum.clone();
        sum_dna.alleles[0].body = AlleleBody::Dna(bh);
        cells.insert(sh, sum_dna);
        let wrapper_src = format!(
            "body {{ codex 1 genome {{ cell:{} as a }} grants {{ }} wires {{ }} budget {{ steps 100 }} lineage none }}\n",
            sh.to_hex()
        );
        let wrapper = match parse_body(&wrapper_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut bodies = BTreeMap::new();
        bodies.insert(bh, body);
        let mut state = match BodyState::new(wrapper, cells, natives(), 1) {
            Verdict::Ok(s) => s.with_bodies(bodies),
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("a", 0, int(1), 0);
        let _ = state.inject("a", 1, int(0), 0);
        match state.run() {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("grant") || r.reason.contains("nested"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(_) => {}
        }
    }

    #[test]
    fn cli_a_reads_before_cli_b() {
        let (body, cells) = sum_cli_body();
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("cli_b", 0, text("3"), 1);
        let _ = state.inject("cli_a", 0, text("2"), 0);
        match state.run() {
            Verdict::Ok(rs) => {
                let first_fire = rs.iter().find_map(|r| r.fired.as_ref());
                assert_eq!(first_fire.map(String::as_str), Some("cli_a"));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
