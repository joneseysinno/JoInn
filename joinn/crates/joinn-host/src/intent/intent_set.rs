//! The intent set a body admits: one address per in-port of ∂(body).

use joinn_dna::{Body, Cell, Direction};
use joinn_frame::{Hash, Verdict};
use joinn_link::membrane;
use std::collections::{BTreeMap, BTreeSet};

use super::Address;

/// One address per in-port of the membrane.
pub fn intent_set(body: &Body, cells: &BTreeMap<Hash, Cell>) -> BTreeSet<Address> {
    let Verdict::Ok(mem) = membrane(body, cells) else {
        return BTreeSet::new();
    };
    mem.into_iter()
        .filter(|port| port.direction == Direction::In)
        .map(|port| port.address)
        .collect()
}
