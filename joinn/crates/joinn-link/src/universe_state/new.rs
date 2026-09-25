//! Build one BodyState per bound alias.

use joinn_frame::Verdict;
use joinn_gate::NativeRegistry;
use joinn_live::BodyState;
use std::collections::BTreeMap;

use crate::bind::Bound;
use crate::capability::grant;
use crate::check_link_types::check_link_types;
use crate::universe::Universe;
use crate::universe_state::UniverseState;

impl UniverseState {
    /// Bodies are already bound by hash. Link types are checked here.
    pub fn new(universe: &Universe, bound: &Bound, natives: NativeRegistry) -> Verdict<Self> {
        match check_link_types(universe, bound) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        let mut budget = 0u64;
        let mut bodies = BTreeMap::new();
        for (alias, (body, cells)) in bound.iter() {
            budget = budget.saturating_add(body.coding.budget_steps);
            let state = match BodyState::new(body.clone(), cells.clone(), natives.clone(), 1) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            bodies.insert(alias.to_owned(), state);
        }
        let mut runtime = crate::capability::LinkRuntime::default();
        for (link_id, to) in &universe.coding.grants {
            match grant(&mut runtime, universe, link_id, to) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        Verdict::Ok(Self {
            universe: universe.clone(),
            bodies,
            runtime,
            budget,
            spent: 0,
        })
    }
}
