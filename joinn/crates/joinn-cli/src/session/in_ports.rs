//! In-ports of ∂(body), in genome declaration order.

use joinn_dna::{Body, Cell, Direction};
use joinn_frame::{Hash, Verdict};
use joinn_link::{BoundaryPort, membrane};
use std::collections::BTreeMap;

/// In-ports of the membrane, genome order, then port position.
pub(in crate::session) fn in_ports(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Vec<BoundaryPort> {
    let Verdict::Ok(set) = membrane(body, cells) else {
        return Vec::new();
    };
    let mut ordered = Vec::new();
    for entry in &body.coding.genome {
        for instance in &entry.instances {
            let mut here: Vec<BoundaryPort> = set
                .iter()
                .filter(|port| {
                    port.direction == Direction::In
                        && port.address.instance.as_str() == instance.as_str()
                })
                .cloned()
                .collect();
            here.sort_by_key(|port| port.address.port);
            ordered.extend(here);
        }
    }
    ordered
}
