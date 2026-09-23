//! Try to fire an instance after delivery.

use joinn_dna::{AlleleBody, Direction, NativeId};
use joinn_frame::{CheckId, Refusal, Subject, Verdict};
use joinn_gate::eval_predicate;
use std::collections::BTreeMap;

use crate::ready::{build_parts_ready, choose_branch_ready};
use crate::report::StepReport;
use crate::slot::{slot_ready, slot_value};
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn try_fire(
        &mut self,
        depth: usize,
        name: &str,
        report: &mut StepReport,
    ) -> Verdict<Option<String>> {
        let Some(act) = self.stack.get(depth) else {
            return Verdict::Refused(Refusal::structural(CheckId::Other, "missing activation"));
        };
        let Some(inst) = act.instances.get(name) else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!("try_fire: unknown instance {name}"),
            ));
        };
        if act.nested && act.boundary.as_deref() == Some(name) {
            return Verdict::Ok(None);
        }
        let forward_ready = inst
            .cell
            .coding
            .contract
            .ports
            .iter()
            .filter(|p| p.direction == Direction::In && p.required)
            .all(|p| slot_ready(inst.slots.get(&p.position)));
        let mut turn_out: Option<u32> = None;
        if !forward_ready {
            for t in &inst.cell.coding.turns {
                if t.from.iter().all(|p| slot_ready(inst.slots.get(p))) {
                    turn_out = Some(t.out);
                    break;
                }
            }
            if turn_out.is_none() {
                return Verdict::Ok(None);
            }
        }
        let (inputs, native_or_dna, ensure_map, consume, out_hint) = {
            let Some(act) = self.stack.get(depth) else {
                return Verdict::Refused(Refusal::structural(CheckId::Other, "missing activation"));
            };
            let Some(inst) = act.instances.get(name) else {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("try_fire: unknown instance {name}"),
                ));
            };
            let mut inputs = BTreeMap::new();
            let consume: Vec<u32> = if let Some(out) = turn_out {
                let Some(t) = inst.cell.coding.turns.iter().find(|t| t.out == out) else {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        format!("{name} missing turn {out}"),
                    ));
                };
                for p in &t.from {
                    if let Some(v) = slot_value(inst.slots.get(p)) {
                        inputs.insert(*p, v);
                    }
                }
                t.from.clone()
            } else {
                for p in &inst.cell.coding.contract.ports {
                    if p.direction != Direction::In {
                        continue;
                    }
                    if let Some(v) = slot_value(inst.slots.get(&p.position)) {
                        inputs.insert(p.position, v);
                    }
                }
                inst.cell
                    .coding
                    .contract
                    .ports
                    .iter()
                    .filter(|p| p.direction == Direction::In)
                    .map(|p| p.position)
                    .collect()
            };
            let Some(allele) = inst.cell.alleles.first() else {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("{name} has no allele"),
                ));
            };
            (
                inputs,
                allele.body.clone(),
                inst.cell.coding.contract.ensure.clone(),
                consume,
                turn_out,
            )
        };
        if let AlleleBody::Dna(h) = &native_or_dna {
            return self.push_dna(depth, name, *h, inputs, consume);
        }
        let forward = match &native_or_dna {
            AlleleBody::Native(id) => id.clone(),
            AlleleBody::Dna(_) => {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    "dna already handled",
                ));
            }
        };
        let native = if let Some(out) = turn_out {
            NativeId(format!("{}.turn{out}", forward.0))
        } else {
            forward
        };
        if native.0 == "prim:choose" && !choose_branch_ready(&inputs) {
            return Verdict::Ok(None);
        }
        if native.0 == "prim:build" && !build_parts_ready(&inputs) {
            return Verdict::Ok(None);
        }
        let Some(oracle) = self.natives.get(&native) else {
            return Verdict::Refused(Refusal {
                check: CheckId::Contract,
                subject: Subject::Allele(native.0.clone()),
                reason: format!("unknown native {}", native.0),
                counterexample: None,
                seed: self.seed,
            });
        };
        let outs = match oracle.apply(&inputs) {
            Verdict::Ok(o) => o,
            Verdict::Refused(mut r) => {
                if r.reason.contains("not an integer") || r.reason.contains("not in") {
                    r.reason = format!(
                        "refused at membrane: {}",
                        inputs
                            .values()
                            .next()
                            .map(|v| format!(
                                "\"{}\" is not in ℤ",
                                v.print_term().trim_matches('"')
                            ))
                            .unwrap_or_else(|| r.reason.clone())
                    );
                    r.check = CheckId::Contract;
                }
                return Verdict::Refused(r);
            }
        };
        for (pos, val) in &outs {
            if let Some(ens) = ensure_map.get(pos) {
                let mut env = BTreeMap::new();
                env.insert("port".into(), val.clone());
                match eval_predicate(ens, &env, &self.frames, self.seed) {
                    Verdict::Ok(true) => report.checks.push(format!("ensure {name}@{pos} ok")),
                    Verdict::Ok(false) => {
                        return Verdict::Refused(Refusal {
                            check: CheckId::Contract,
                            subject: Subject::Other(name.into()),
                            reason: format!("ensure failed at {name}@{pos}"),
                            counterexample: None,
                            seed: self.seed,
                        });
                    }
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            } else {
                report.checks.push(format!("ensure {name}@{pos} ok"));
            }
        }
        self.consume_and_emit(depth, name, &consume, &outs);
        let mut ports = inputs;
        for (pos, val) in &outs {
            ports.insert(*pos, val.clone());
        }
        self.last_ports.insert(name.into(), ports);
        report.direction = Some(out_hint.unwrap_or(0));
        self.enqueue_outs(depth, name, &outs);
        self.advance_grants(depth, name);
        Verdict::Ok(Some(name.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{int, natives};
    use joinn_dna::{TurnDecl, hash, parse_body, sum_cell};
    use joinn_frame::FrameRegistry;
    use std::collections::BTreeMap;

    #[test]
    fn five_three_at_declared_turn() {
        let mut sum = sum_cell();
        sum.coding.turns = vec![TurnDecl {
            out: 0,
            from: vec![1, 2],
        }];
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
        let _ = state.inject("sum", 1, int(3), 0);
        let _ = state.inject("sum", 2, int(5), 0);
        match state.run() {
            Verdict::Ok(rs) => {
                assert!(
                    rs.iter()
                        .any(|r| r.direction == Some(0) && r.fired.as_deref() == Some("sum")),
                    "turn did not fire"
                );
                assert_eq!(state.slot("sum", 0), Some(&int(2)));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
