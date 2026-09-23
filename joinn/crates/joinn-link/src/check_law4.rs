//! Inside a body is a wire; between bodies is a hyperedge.
//! allow(modules): Law 4 returns every refusal, and check takes the first

use joinn_frame::{CheckId, Refusal, Verdict};
use std::collections::BTreeSet;

use crate::universe::Universe;

/// Every Law 4 refusal for this universe. Empty when Law 4 holds.
pub fn law4_refusals(universe: &Universe) -> Vec<Refusal> {
    let mut out = Vec::new();
    for link in &universe.coding.links {
        let bodies: BTreeSet<&str> = link.members.iter().map(|m| m.body.as_str()).collect();
        if bodies.len() == 1 {
            if let Some(body) = bodies.iter().next() {
                out.push(Refusal::structural(
                    CheckId::Other,
                    format!(
                        "link {} has all members in body {body}; acceptance is a wire inside body {body}",
                        link.id
                    ),
                ));
            }
        }
    }
    for wire in &universe.coding.cross_wires {
        out.push(Refusal::structural(
            CheckId::Other,
            format!(
                "wire {}@{} -> {}@{} spans bodies {} and {}; acceptance is a hyperedge between those bodies",
                format!("{}.{}", wire.src_body, wire.src_instance),
                wire.src_port,
                format!("{}.{}", wire.dst_body, wire.dst_instance),
                wire.dst_port,
                wire.src_body,
                wire.dst_body
            ),
        ));
    }
    out
}

/// Refuse the first Law 4 violation.
pub fn check_law4(universe: &Universe) -> Verdict<()> {
    match law4_refusals(universe).into_iter().next() {
        Some(r) => Verdict::Refused(r),
        None => Verdict::Ok(()),
    }
}
