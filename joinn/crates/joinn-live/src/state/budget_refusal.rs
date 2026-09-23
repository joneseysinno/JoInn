//! Budget exceeded refusal.

use joinn_frame::{CheckId, Refusal, Subject, Verdict};

use crate::report::StepReport;
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn budget_refusal(&self) -> Verdict<StepReport> {
        let path = self.activation_path();
        let last = self.last_mail.clone().unwrap_or_else(|| "none".into());
        Verdict::Refused(Refusal {
            check: CheckId::Budget,
            subject: Subject::Other(path.clone()),
            reason: format!(
                "budget exceeded: steps {} path {path} last {last} seed {}",
                self.step, self.seed
            ),
            counterexample: None,
            seed: self.seed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{int, natives, tagged_sum};
    use joinn_dna::{AlleleBody, hash, parse_body, sum_cell};
    use joinn_frame::{CheckId, FrameRegistry};
    use std::collections::BTreeMap;

    #[test]
    fn budget_refusal_is_distinct() {
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let src = format!(
            "body {{ codex 1 genome {{ cell:{} as c }} grants {{ }} wires {{ c@2 -> c@0 }} budget {{ steps 3 }} lineage none }}\n",
            sh.to_hex()
        );
        let body = match parse_body(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut cells = BTreeMap::new();
        cells.insert(sh, sum);
        let mut state = match BodyState::new(body, cells, natives(), 11) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("c", 0, int(1), 0);
        let _ = state.inject("c", 1, int(1), 0);
        match state.run() {
            Verdict::Refused(r) => {
                assert_eq!(r.check, CheckId::Budget);
                assert!(r.reason.contains("steps"), "{}", r.reason);
                assert!(r.reason.contains("seed 11"), "{}", r.reason);
            }
            Verdict::Ok(_) => panic!("ungarded self-wire must hit budget"),
        }
    }

    #[test]
    fn nested_budget_names_activation_path() {
        let mut a_cell = tagged_sum(11);
        let mut b_cell = tagged_sum(12);
        let mut c_cell = tagged_sum(13);
        let ah = hash(&a_cell.coding);
        let bh = hash(&b_cell.coding);
        let ch = hash(&c_cell.coding);
        assert_ne!(ah, bh);
        assert_ne!(bh, ch);
        let body_c_src = format!(
            "body {{ codex 1 genome {{ cell:{} as self prim:eq as e }} grants {{ }} wires {{ self@0 -> e@0 self@0 -> e@1 e@0 -> e@0 e@0 -> e@1 }} budget {{ steps 100 }} lineage none }}\n",
            ch.to_hex()
        );
        let body_c = match parse_body(&body_c_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let ch_body = hash(&body_c.coding);
        let body_b_src = format!(
            "body {{ codex 1 genome {{ cell:{} as self cell:{} as c }} grants {{ }} wires {{ self@0 -> c@0 self@1 -> c@1 c@2 -> self@2 }} budget {{ steps 100 }} lineage none }}\n",
            bh.to_hex(),
            ch.to_hex()
        );
        let body_b = match parse_body(&body_b_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let bh_body = hash(&body_b.coding);
        let body_a_src = format!(
            "body {{ codex 1 genome {{ cell:{} as self cell:{} as b }} grants {{ }} wires {{ self@0 -> b@0 self@1 -> b@1 b@2 -> self@2 }} budget {{ steps 100 }} lineage none }}\n",
            ah.to_hex(),
            bh.to_hex()
        );
        let body_a = match parse_body(&body_a_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let ah_body = hash(&body_a.coding);
        a_cell.alleles[0].body = AlleleBody::Dna(ah_body);
        b_cell.alleles[0].body = AlleleBody::Dna(bh_body);
        c_cell.alleles[0].body = AlleleBody::Dna(ch_body);
        let wrapper_src = format!(
            "body {{ codex 1 genome {{ cell:{} as a }} grants {{ }} wires {{ }} budget {{ steps 3 }} lineage none }}\n",
            ah.to_hex()
        );
        let wrapper = match parse_body(&wrapper_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut cells = BTreeMap::new();
        cells.insert(ah, a_cell);
        cells.insert(bh, b_cell);
        cells.insert(ch, c_cell);
        let mut bodies = BTreeMap::new();
        bodies.insert(ah_body, body_a);
        bodies.insert(bh_body, body_b);
        bodies.insert(ch_body, body_c);
        let mut state = match BodyState::new(wrapper, cells, natives(), 1) {
            Verdict::Ok(s) => s.with_bodies(bodies),
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("a", 0, int(1), 0);
        let _ = state.inject("a", 1, int(1), 0);
        match state.run() {
            Verdict::Refused(r) => {
                assert_eq!(r.check, CheckId::Budget);
                assert!(r.reason.contains("▸"), "budget path: {}", r.reason);
                assert!(r.reason.contains('a'), "{}", r.reason);
                assert!(r.reason.contains('b'), "{}", r.reason);
                assert!(r.reason.contains('c'), "{}", r.reason);
            }
            Verdict::Ok(_) => panic!("nested budget must refuse naming the path"),
        }
    }
}
