//! The single delivery path (V35).

use joinn_dna::Direction;
use joinn_frame::{CheckId, Refusal, Subject, Verdict};
use joinn_gate::eval_predicate;
use std::collections::BTreeMap;

use crate::activation::Mail;
use crate::report::StepReport;
use crate::slot::{SlotWrite, apply_slot};
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn deliver(&mut self, depth: usize, mail: Mail) -> Verdict<StepReport> {
        self.deliveries += 1;
        let mut report = StepReport {
            step: self.step,
            fired: None,
            direction: None,
            delivered: vec![(
                format!("{}@{}", mail.dest, mail.port),
                mail.value.print_literal(),
            )],
            checks: Vec::new(),
            depth: self.depth(),
        };
        let dest = mail.dest.clone();
        let port = mail.port;
        self.refusal_site = Some((dest.clone(), port));
        let value = mail.value.clone();
        let seed = self.seed;
        {
            let Some(act) = self.stack.get(depth) else {
                return Verdict::Refused(Refusal::structural(CheckId::Other, "missing activation"));
            };
            let Some(inst) = act.instances.get(&dest) else {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("deliver: unknown instance {dest}"),
                ));
            };
            let Some(pdecl) = inst
                .cell
                .coding
                .contract
                .ports
                .iter()
                .find(|p| p.position == port)
            else {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("deliver: no port {dest}@{port}"),
                ));
            };
            if pdecl.direction == Direction::In && !inst.is_prim && value.frame() != &pdecl.frame {
                return Verdict::Refused(Refusal {
                    check: CheckId::Contract,
                    subject: Subject::Other(dest),
                    reason: format!(
                        "refused at membrane: {} is not in {}",
                        value.print_term(),
                        pdecl.frame
                    ),
                    counterexample: None,
                    seed,
                });
            }
            if let Some(req) = inst.cell.coding.contract.require.get(&port) {
                let mut env = BTreeMap::new();
                env.insert("port".into(), value.clone());
                match eval_predicate(req, &env, &self.frames, seed) {
                    Verdict::Ok(true) => report.checks.push(format!("require {dest}@{port} ok")),
                    Verdict::Ok(false) => {
                        return Verdict::Refused(Refusal {
                            check: CheckId::Contract,
                            subject: Subject::Other(dest),
                            reason: format!(
                                "refused at membrane: {} is not in {}",
                                value.print_term(),
                                pdecl.frame
                            ),
                            counterexample: None,
                            seed,
                        });
                    }
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            } else {
                report.checks.push(format!("require {dest}@{port} ok"));
            }
        }
        let boundary = self.stack.get(depth).and_then(|a| a.boundary.clone());
        let nested = self.stack.get(depth).map(|a| a.nested).unwrap_or(false);
        let policy = match self.stack.get(depth).and_then(|a| a.instances.get(&dest)) {
            Some(inst) => inst.cell.coding.contract.join_policy,
            None => {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("deliver: unknown instance {dest}"),
                ));
            }
        };
        {
            let Some(act) = self.stack.get_mut(depth) else {
                return Verdict::Refused(Refusal::structural(CheckId::Other, "missing activation"));
            };
            let Some(inst) = act.instances.get_mut(&dest) else {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("deliver: unknown instance {dest}"),
                ));
            };
            match apply_slot(inst, port, SlotWrite::Fill(value), policy, &dest, seed) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        if nested && boundary.as_deref() == Some(dest.as_str()) {
            let is_out = self
                .stack
                .get(depth)
                .and_then(|a| a.instances.get(&dest))
                .and_then(|inst| {
                    inst.cell
                        .coding
                        .contract
                        .ports
                        .iter()
                        .find(|p| p.position == port)
                })
                .is_some_and(|p| p.direction == Direction::Out);
            if is_out {
                return Verdict::Ok(report);
            }
        }
        match self.try_fire(depth, &dest, &mut report) {
            Verdict::Ok(Some(name)) => report.fired = Some(name),
            Verdict::Ok(None) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        Verdict::Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{int, natives, sum_cli_body, text};
    use joinn_dna::{JoinPolicy, hash, parse_body, sum_cell};
    use joinn_frame::{CheckId, FrameRegistry};
    use std::collections::BTreeMap;

    #[test]
    fn two_into_cli_refuses_at_membrane_and_does_not_cross() {
        let (body, cells) = sum_cli_body();
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("cli_a", 0, text("two"), 0);
        match state.run() {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("membrane") || r.reason.contains("two"),
                    "{}",
                    r.reason
                );
                assert_ne!(r.check, CheckId::Budget);
            }
            Verdict::Ok(_) => panic!("\"two\" must refuse"),
        }
        assert!(state.slot("sum", 0).is_none());
        assert!(state.slot("cli_a", 1).is_none());
        let _ = state.inject("cli_a", 0, text("2"), 0);
        let _ = state.inject("cli_b", 0, text("3"), 1);
        match state.run() {
            Verdict::Ok(_) => {
                assert_eq!(state.slot("sum", 2), Some(&int(5)));
            }
            Verdict::Refused(r) => panic!("after recovery: {}", r.reason),
        }
    }

    #[test]
    fn join_refuse_names_port_and_policy() {
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let src = format!(
            "body {{ codex 1 genome {{ cell:{} as sum }} grants {{ }} wires {{ }} budget {{ steps 100 }} lineage none }}\n",
            sh.to_hex()
        );
        let body = match parse_body(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut cells = BTreeMap::new();
        cells.insert(sh, sum);
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("sum", 0, int(1), 0);
        let _ = state.inject("sum", 0, int(2), 0);
        match state.run() {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("refuse"), "{}", r.reason);
                assert!(r.reason.contains("port"), "{}", r.reason);
                assert_eq!(r.check, CheckId::Join);
            }
            Verdict::Ok(_) => panic!("second message must refuse"),
        }
    }

    #[test]
    fn latest_replaces_and_queue_queues() {
        fn one_sum(
            policy: JoinPolicy,
        ) -> (
            joinn_dna::Body,
            BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
        ) {
            let mut sum = sum_cell();
            sum.coding.contract.join_policy = policy;
            let sh = hash(&sum.coding);
            let src = format!(
                "body {{ codex 1 genome {{ cell:{} as sum }} grants {{ }} wires {{ }} budget {{ steps 100 }} lineage none }}\n",
                sh.to_hex()
            );
            let body = match parse_body(&src, &FrameRegistry::phase1()) {
                Verdict::Ok(b) => b,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let mut cells = BTreeMap::new();
            cells.insert(sh, sum);
            (body, cells)
        }
        let (body, cells) = one_sum(JoinPolicy::Latest);
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("sum", 0, int(1), 0);
        let _ = state.inject("sum", 0, int(2), 0);
        let _ = state.inject("sum", 1, int(3), 0);
        match state.run() {
            Verdict::Ok(_) => {
                assert_eq!(state.slot("sum", 2), Some(&int(5)));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }

        let (body, cells) = one_sum(JoinPolicy::Queue);
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("sum", 0, int(1), 0);
        let _ = state.inject("sum", 0, int(2), 0);
        let _ = state.inject("sum", 1, int(3), 0);
        match state.run() {
            Verdict::Ok(_) => {
                assert_eq!(state.slot("sum", 2), Some(&int(4)));
                assert_eq!(state.slot("sum", 0), Some(&int(2)));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
