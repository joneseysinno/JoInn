//! Build one BodyState per bound alias.

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_gate::NativeRegistry;
use joinn_live::BodyState;
use std::collections::BTreeMap;

use crate::check_link_types::check_link_types;
use crate::universe::Universe;
use crate::universe_state::UniverseState;

impl UniverseState {
    /// Bodies are already bound by hash. Link types are checked here.
    pub fn new(
        universe: &Universe,
        bound: BTreeMap<String, (Body, BTreeMap<Hash, Cell>)>,
        natives: NativeRegistry,
    ) -> Verdict<Self> {
        match check_link_types(universe, &bound) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        let mut budget = 0u64;
        let mut bodies = BTreeMap::new();
        for (alias, (body, cells)) in bound {
            budget = budget.saturating_add(body.coding.budget_steps);
            let state = match BodyState::new(body, cells, natives.clone(), 1) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            bodies.insert(alias, state);
        }
        Verdict::Ok(Self {
            universe: universe.clone(),
            bodies,
            runtime: crate::capability::LinkRuntime::default(),
            budget,
            spent: 0,
        })
    }
}
