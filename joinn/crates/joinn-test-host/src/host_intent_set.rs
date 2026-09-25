//! Membrane in-ports of a body that no link head feeds.

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use joinn_host::{Address, intent_set};
use joinn_link::{Mark, Universe};
use std::collections::{BTreeMap, BTreeSet};

/// `intent_set(body)` without ports that are heads of a link on `alias`.
pub fn host_intent_set(
    universe: &Universe,
    alias: &str,
    body: &Body,
    cells: &BTreeMap<Hash, Cell>,
) -> BTreeSet<Address> {
    let mut set = intent_set(body, cells);
    for link in &universe.coding.links {
        for member in &link.members {
            if member.mark == Mark::Head && member.body == alias {
                set.remove(&Address {
                    instance: member.instance.clone(),
                    port: member.port,
                });
            }
        }
    }
    set
}
