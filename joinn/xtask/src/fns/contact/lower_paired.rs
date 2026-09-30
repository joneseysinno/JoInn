//! `lower`, with one force's members assigned to its in-ports in a given order.

use joinn_dna::{Body, Cell, Contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::lower;
use std::collections::BTreeMap;

/// `orders[name][i]` is the in-port that canonical member `i` of force `name`
/// feeds; a force not in `orders` keeps the canonical order.
pub(super) fn lower_paired(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
    orders: &BTreeMap<String, Vec<u32>>,
) -> Verdict<Body> {
    let mut body = match lower(contact, cells, frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for wire in &mut body.coding.wires {
        if let Some(order) = orders.get(&wire.dst_instance)
            && let Some(port) = usize::try_from(wire.dst_port)
                .ok()
                .and_then(|i| order.get(i))
        {
            wire.dst_port = *port;
        }
    }
    Verdict::Ok(body)
}
