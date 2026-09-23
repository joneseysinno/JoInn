//! Output port of a cell.

use joinn_dna::{Cell, Direction};

pub(in crate::seals) fn out_port(cell: &Cell) -> u32 {
    cell.coding
        .contract
        .ports
        .iter()
        .find(|p| p.direction == Direction::Out)
        .map(|p| p.position)
        .unwrap_or(0)
}
