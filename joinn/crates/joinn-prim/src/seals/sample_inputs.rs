//! Sample all input ports for a cell.

use joinn_dna::{Cell, Direction};
use joinn_frame::Value;
use std::collections::BTreeMap;

use super::Drive;
use super::sample_port::sample_port;

pub(in crate::seals) fn sample_inputs(
    cell: &Cell,
    drive: &Drive,
    seed: u64,
    i: u32,
) -> BTreeMap<u32, Value> {
    let mut inputs = BTreeMap::new();
    for p in &cell.coding.contract.ports {
        if p.direction != Direction::In {
            continue;
        }
        let jitter = seed
            .wrapping_add(u64::from(i).wrapping_mul(17))
            .wrapping_add(u64::from(p.position).wrapping_mul(31));
        inputs.insert(
            p.position,
            sample_port(p.frame, jitter, i, drive, p.position),
        );
    }
    inputs
}
