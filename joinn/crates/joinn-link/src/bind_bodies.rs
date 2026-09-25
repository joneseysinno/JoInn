//! Bind universe aliases to bodies by coding hash.

use joinn_dna::{Body, Cell, hash};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use crate::universe::Universe;

/// One body and the cells it names, keyed for binding.
type BodyWithCells = (Body, BTreeMap<Hash, Cell>);

/// Alias → body, looked up by the binding's declared coding hash.
pub fn bind_bodies(
    universe: &Universe,
    supplied: &BTreeMap<Hash, BodyWithCells>,
) -> Verdict<BTreeMap<String, BodyWithCells>> {
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
