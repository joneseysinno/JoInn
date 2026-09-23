//! Within one lens every body has exactly one owning system.

use joinn_frame::{CheckId, Refusal, Verdict};
use std::collections::BTreeMap;

use crate::universe::Universe;

/// Refuse a body that appears in two systems of the same lens.
pub fn check_lenses(universe: &Universe) -> Verdict<()> {
    for lens in &universe.coding.lenses {
        let mut owner: BTreeMap<&str, &str> = BTreeMap::new();
        for galaxy in &lens.galaxies {
            for system in &galaxy.systems {
                for alias in &system.bodies {
                    if let Some(first) = owner.insert(alias.as_str(), system.name.as_str()) {
                        if first != system.name.as_str() {
                            return Verdict::Refused(Refusal::structural(
                                CheckId::Other,
                                format!(
                                    "body {alias} belongs to systems {first} and {} in lens {}; acceptance is one system per body per lens",
                                    system.name, lens.name
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }
    Verdict::Ok(())
}
