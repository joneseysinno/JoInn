//! Build a complex from a universe and call assemble. Touch-only is membership in C0.
//! allow(modules): maps assemble refusals back to port addresses and wires

use joinn_assay::{BlockId, Chain, Complex};
use joinn_dna::{Body, Cell};
use joinn_frame::{CheckId, Hash, Refusal, Verdict};
use std::collections::BTreeMap;

use crate::membrane::membrane;
use crate::universe::{Mark, Universe};

/// Assemble a universe: every link member must lie on a membrane.
pub fn assemble_universe(
    universe: &Universe,
    bodies: &BTreeMap<String, (Body, BTreeMap<Hash, Cell>)>,
) -> Verdict<()> {
    let mut dimension = BTreeMap::new();
    let mut boundaries = BTreeMap::new();
    let mut port_of: BTreeMap<BlockId, (String, String, u32)> = BTreeMap::new();
    let mut block_of: BTreeMap<(String, String, u32), BlockId> = BTreeMap::new();
    let mut next = 0u32;

    for binding in &universe.coding.bodies {
        let alias = &binding.alias;
        let Some((body, cells)) = bodies.get(alias) else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Other,
                format!(
                    "body alias {} has no loaded body; acceptance is a body for every alias",
                    binding.alias
                ),
            ));
        };
        let mem = match membrane(body, cells) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        for port in &mem {
            let id = BlockId(next);
            next = next.saturating_add(1);
            dimension.insert(id, 0);
            boundaries.insert(id, Chain::from_coeffs([]));
            port_of.insert(
                id,
                (
                    binding.alias.clone(),
                    port.address.instance.clone(),
                    port.address.port,
                ),
            );
            block_of.insert(
                (
                    binding.alias.clone(),
                    port.address.instance.clone(),
                    port.address.port,
                ),
                id,
            );
        }
    }

    let mut link_of: BTreeMap<BlockId, String> = BTreeMap::new();
    for link in &universe.coding.links {
        let link_id = BlockId(next);
        next = next.saturating_add(1);
        link_of.insert(link_id, link.id.clone());
        let mut coeffs = Vec::new();
        for member in &link.members {
            let key = (
                member.body.clone(),
                member.instance.clone(),
                member.port,
            );
            let face = match block_of.get(&key) {
                Some(id) => *id,
                None => {
                    let id = BlockId(next);
                    next = next.saturating_add(1);
                    port_of.insert(id, key.clone());
                    id
                }
            };
            let sign = match member.mark {
                Mark::Head => 1,
                Mark::Tail => -1,
                Mark::None => 1,
            };
            coeffs.push((face, sign));
        }
        dimension.insert(link_id, 1);
        boundaries.insert(link_id, Chain::from_coeffs(coeffs));
    }

    let complex = Complex::from_parts(dimension, boundaries);
    match complex.assemble() {
        Verdict::Ok(()) => Verdict::Ok(()),
        Verdict::Refused(r) => {
            Verdict::Refused(translate_refusal(r, &port_of, bodies))
        }
    }
}

fn translate_refusal(
    refusal: Refusal,
    port_of: &BTreeMap<BlockId, (String, String, u32)>,
    bodies: &BTreeMap<String, (Body, BTreeMap<Hash, Cell>)>,
) -> Refusal {
    let reason = &refusal.reason;
    let Some(endpoint) = reason
        .split_whitespace()
        .skip_while(|w| *w != "endpoint")
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
    else {
        return refusal;
    };
    let Some((alias, instance, port)) = port_of.get(&BlockId(endpoint)) else {
        return refusal;
    };
    let printed = format!("{alias}.{instance}@{port}");
    let wire = match bodies.get(alias) {
        Some((body, _)) => consuming_wire(body, instance, *port),
        None => None,
    };
    let detail = match wire {
        Some(w) => format!("{printed} is interior: consumed by wire {w}"),
        None => format!(
            "no such port {printed}; acceptance is a declared port on ∂({alias})"
        ),
    };
    Refusal::structural(CheckId::Other, detail)
}

fn consuming_wire(body: &Body, instance: &str, port: u32) -> Option<String> {
    for wire in &body.coding.wires {
        if wire.src_instance == instance && wire.src_port == port {
            return Some(format!(
                "{}@{} -> {}@{}",
                wire.src_instance, wire.src_port, wire.dst_instance, wire.dst_port
            ));
        }
        if wire.dst_instance == instance && wire.dst_port == port {
            return Some(format!(
                "{}@{} -> {}@{}",
                wire.src_instance, wire.src_port, wire.dst_instance, wire.dst_port
            ));
        }
    }
    None
}
