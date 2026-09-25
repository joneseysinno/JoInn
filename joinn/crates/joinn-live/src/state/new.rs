//! Load a body with the cells it names.

use joinn_dna::{Body, Cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_gate::NativeRegistry;
use std::collections::BTreeMap;

use crate::activation::build_activation;
use crate::state::BodyState;

impl BodyState {
    pub fn new(
        body: Body,
        cells: BTreeMap<Hash, Cell>,
        natives: NativeRegistry,
        seed: u64,
    ) -> Verdict<Self> {
        match joinn_dna::check_body(&body, &cells) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        let budget = body.coding.budget_steps;
        let act = match build_activation(&body, &cells, None, "root", false, seed) {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        Verdict::Ok(Self {
            stack: vec![act],
            natives,
            frames: FrameRegistry::phase1(),
            cells,
            bodies: BTreeMap::new(),
            step: 0,
            seq: 0,
            seed,
            last_mail: None,
            deliveries: 0,
            last_ports: BTreeMap::new(),
            budget,
            last_refusal: None,
            refusal_site: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{natives, sum_cli_body, text};

    #[test]
    fn one_delivery_path() {
        let (body, cells) = sum_cli_body();
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("cli_a", 0, text("2"), 0);
        let _ = state.inject("cli_b", 0, text("3"), 1);
        let _ = state.run();
        assert!(state.delivery_count() >= 1);
    }
}
