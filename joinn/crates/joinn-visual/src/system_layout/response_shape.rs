//! A response cell's height and out-port.

use joinn_dna::{Cell, Direction, SystemForce};
use joinn_frame::Hash;
use std::collections::BTreeMap;

/// The response's height (by its ports, as `layout_contact` sizes a cell) and
/// its out-port.
pub(super) fn response_shape(
    force: &SystemForce,
    cells: &BTreeMap<Hash, Cell>,
) -> Option<(i64, u32)> {
    let cell = cells.get(&force.response)?;
    let ports = &cell.coding.contract.ports;
    let ins = ports
        .iter()
        .filter(|p| p.direction == Direction::In)
        .count();
    let outs: Vec<u32> = ports
        .iter()
        .filter(|p| p.direction == Direction::Out)
        .map(|p| p.position)
        .collect();
    let rows = i64::try_from(ins.max(outs.len())).ok()?.max(1);
    Some((
        crate::layout::PORT_INSET * 2 + crate::layout::PITCH * (rows - 1),
        *outs.first()?,
    ))
}
