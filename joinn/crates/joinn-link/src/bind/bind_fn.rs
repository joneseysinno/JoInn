//! Match each universe binding to a body the store holds at the declared hash.

use joinn_frame::Verdict;
use std::collections::BTreeMap;

use super::Bound;
use crate::body_store::BodyStore;
use crate::universe::Universe;

/// Match each universe binding to a body the store holds at the declared hash.
pub fn bind(universe: &Universe, store: &BodyStore) -> Verdict<Bound> {
    let mut by_alias = BTreeMap::new();
    for binding in &universe.coding.bodies {
        let Some((body, cells)) = store.get(&binding.hash) else {
            return crate::refuse(format!(
                "alias {} declared {} and the store holds no such body",
                binding.alias,
                binding.hash.to_hex()
            ));
        };
        by_alias.insert(binding.alias.clone(), (body.clone(), cells.clone()));
    }
    Verdict::Ok(Bound { by_alias })
}
