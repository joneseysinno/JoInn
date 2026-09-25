//! Bind universe aliases to bodies by coding hash.

use joinn_dna::{Body, Cell, hash};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use crate::universe::Universe;

/// Alias → body, looked up by the binding's declared coding hash.
pub fn bind_bodies(
    universe: &Universe,
    supplied: &BTreeMap<Hash, (Body, BTreeMap<Hash, Cell>)>,
) -> Verdict<BTreeMap<String, (Body, BTreeMap<Hash, Cell>)>> {
    let mut bound = BTreeMap::new();
    for binding in &universe.coding.bodies {
        let Some((body, cells)) = supplied.get(&binding.hash) else {
            return crate::refuse(format!(
                "alias {} declared {} and no body was supplied; acceptance is that coding hash",
                binding.alias,
                binding.hash.short_hex()
            ));
        };
        let got = hash(&body.coding);
        if got != binding.hash {
            return crate::refuse(format!(
                "alias {} declared {} supplied {}; acceptance is the declared coding hash",
                binding.alias,
                binding.hash.short_hex(),
                got.short_hex()
            ));
        }
        bound.insert(binding.alias.clone(), (body.clone(), cells.clone()));
    }
    Verdict::Ok(bound)
}
