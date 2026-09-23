//! `DnaFire` for `LiveDna`.

use joinn_dna::{AlleleBody, Body, Cell, Direction, hash};
use joinn_frame::{CheckId, FrameRegistry, Hash, Refusal, Value, Verdict};
use std::collections::BTreeMap;

use crate::dna::LiveDna;
use crate::state::BodyState;

impl joinn_prim::DnaFire for LiveDna {
    fn fire(
        &self,
        body: &Body,
        cell: &Cell,
        cells: &BTreeMap<Hash, Cell>,
        bodies: &BTreeMap<Hash, Body>,
        inputs: &BTreeMap<u32, Value>,
    ) -> Verdict<BTreeMap<u32, Value>> {
        let bh = hash(&body.coding);
        let ch = hash(&cell.coding);
        let mut cell_dna = cell.clone();
        if let Some(a) = cell_dna.alleles.first_mut() {
            a.body = AlleleBody::Dna(bh);
        }
        let wrapper_src = format!(
            "body {{ codex 1 genome {{ cell:{} as under }} grants {{ }} wires {{ }} budget {{ steps {} }} lineage none }}\n",
            ch.to_hex(),
            body.coding.budget_steps.max(1)
        );
        let wrapper = match joinn_dna::parse_body(&wrapper_src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut cells = cells.clone();
        cells.insert(ch, cell_dna);
        let mut bodies = bodies.clone();
        bodies.insert(bh, body.clone());
        let mut state = match BodyState::new(wrapper, cells, self.natives.clone(), 1) {
            Verdict::Ok(s) => s.with_bodies(bodies),
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        for (p, v) in inputs {
            match state.inject("under", *p, v.clone(), 0) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        match state.run() {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        self.last_steps.set(state.steps());
        match state.last_ports("under") {
            Some(ports) => {
                let mut outs = BTreeMap::new();
                for p in &cell.coding.contract.ports {
                    if p.direction == Direction::Out {
                        if let Some(v) = ports.get(&p.position) {
                            outs.insert(p.position, v.clone());
                        }
                    }
                }
                if outs.is_empty() {
                    Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        "Dna fire produced no out-ports",
                    ))
                } else {
                    Verdict::Ok(outs)
                }
            }
            None => Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                "Dna fire did not fire the cell",
            )),
        }
    }

    fn last_steps(&self) -> u64 {
        self.last_steps.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{int, natives, print_body_eq, text};
    use joinn_dna::{cli_input_cell, format_cell, parse_body, sum_cell};
    use joinn_prim::DnaFire;

    #[test]
    fn dna_add_matches_native_on_two_plus_three() {
        let src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase21/int_add_ref.body"
        ));
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let printed = joinn_dna::print_body(&body.coding);
        let parsed_again = match parse_body(&printed, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(print_body_eq(&body), print_body_eq(&parsed_again));
        let got = hash(&body.coding).to_hex();
        const HAND: &str = "50ce5a875106e2731cddf6c09a3373b126498335740fe677a56dd9dc3add7a89";
        assert_eq!(got, HAND, "int_add_ref.body hash {got}");
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let mut cells = BTreeMap::new();
        cells.insert(sh, sum.clone());
        let dna = LiveDna::new(natives());
        let inputs = BTreeMap::from([(0, int(2)), (1, int(3))]);
        match dna.fire(&body, &sum, &cells, &BTreeMap::new(), &inputs) {
            Verdict::Ok(outs) => {
                assert_eq!(outs.get(&2), Some(&int(5)), "add(2,3) via DNA");
            }
            Verdict::Refused(r) => panic!("dna add refused: {}", r.reason),
        }
        match dna.fire(
            &body,
            &sum,
            &cells,
            &BTreeMap::new(),
            &BTreeMap::from([(0, int(0)), (1, int(0))]),
        ) {
            Verdict::Ok(outs) => assert_eq!(outs.get(&2), Some(&int(0))),
            Verdict::Refused(r) => panic!("dna add(0,0): {}", r.reason),
        }
        match dna.fire(
            &body,
            &sum,
            &cells,
            &BTreeMap::new(),
            &BTreeMap::from([(0, int(4)), (1, int(-2))]),
        ) {
            Verdict::Ok(outs) => assert_eq!(outs.get(&2), Some(&int(2))),
            Verdict::Refused(r) => panic!("dna add(4,-2): {}", r.reason),
        }
    }

    #[test]
    fn dna_mul_matches_native_on_small_values() {
        let src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase22/int_mul_ref.body"
        ));
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mul_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase21/mul.cell"
        ));
        let mulc = match joinn_dna::parse_cell(mul_src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mh = hash(&mulc.coding);
        let turn_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase2/sum_turn.cell"
        ));
        let turn = match joinn_dna::parse_cell(turn_src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let th = hash(&turn.coding);
        let mut cells = BTreeMap::new();
        cells.insert(mh, mulc.clone());
        cells.insert(th, turn);
        let dna = LiveDna::new(natives());
        for (a, b, p) in [(0, 0, 0), (2, 3, 6), (4, -2, -8), (-3, -3, 9), (5, 1, 5)] {
            let inputs = BTreeMap::from([(0, int(a)), (1, int(b))]);
            match dna.fire(&body, &mulc, &cells, &BTreeMap::new(), &inputs) {
                Verdict::Ok(outs) => {
                    assert_eq!(outs.get(&2), Some(&int(p)), "mul({a},{b})");
                }
                Verdict::Refused(r) => panic!("dna mul({a},{b}): {}", r.reason),
            }
        }
    }

    #[test]
    fn dna_format_zero_is_zero_text() {
        let src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase22/int_format_ref.body"
        ));
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let fmt = format_cell();
        let fh = hash(&fmt.coding);
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let turn_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase2/sum_turn.cell"
        ));
        let turn = match joinn_dna::parse_cell(turn_src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let th = hash(&turn.coding);
        let mut cells = BTreeMap::new();
        cells.insert(fh, fmt.clone());
        cells.insert(sh, sum);
        cells.insert(th, turn);
        let dna = LiveDna::new(natives());
        for (n, want) in [(0, "0"), (10, "10"), (-1, "-1"), (32, "32")] {
            match dna.fire(
                &body,
                &fmt,
                &cells,
                &BTreeMap::new(),
                &BTreeMap::from([(0, int(n))]),
            ) {
                Verdict::Ok(outs) => assert_eq!(outs.get(&1), Some(&text(want)), "format({n})"),
                Verdict::Refused(r) => panic!("format({n}): {}", r.reason),
            }
            assert!(dna.last_steps() > 0, "format({n}) took zero steps");
        }
    }

    #[test]
    fn dna_parse_samples() {
        let src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase22/text_parse_ref.body"
        ));
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let cli = cli_input_cell(hash(&format_cell().coding));
        let ch = hash(&cli.coding);
        let sum = sum_cell();
        let sh = hash(&sum.coding);
        let mul_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase21/mul.cell"
        ));
        let mulc = match joinn_dna::parse_cell(mul_src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mh = hash(&mulc.coding);
        let turn_src = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../corpus/phase2/sum_turn.cell"
        ));
        let turn = match joinn_dna::parse_cell(turn_src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let th = hash(&turn.coding);
        let mut cells = BTreeMap::new();
        cells.insert(ch, cli.clone());
        cells.insert(sh, sum);
        cells.insert(mh, mulc);
        cells.insert(th, turn);
        let dna = LiveDna::new(natives());
        for (s, n) in [
            ("0", 0),
            ("10", 10),
            ("-1", -1),
            ("32", 32),
            ("1", 1),
            ("111", 111),
        ] {
            match dna.fire(
                &body,
                &cli,
                &cells,
                &BTreeMap::new(),
                &BTreeMap::from([(0, text(s))]),
            ) {
                Verdict::Ok(outs) => assert_eq!(outs.get(&1), Some(&int(n)), "parse({s})"),
                Verdict::Refused(r) => panic!("parse({s}): {}", r.reason),
            }
        }
        match dna.fire(
            &body,
            &cli,
            &cells,
            &BTreeMap::new(),
            &BTreeMap::from([(0, text("007"))]),
        ) {
            Verdict::Refused(_) => {}
            Verdict::Ok(o) => panic!("parse(\"007\") must refuse, got {o:?}"),
        }
    }
}
