//! Frame of a drive port.

use joinn_dna::Cell;
use joinn_frame::FrameRef;

pub(in crate::seals) fn drive_frame(cell: &Cell, port: u32) -> FrameRef {
    cell.coding
        .contract
        .ports
        .iter()
        .find(|p| p.position == port)
        .map(|p| p.frame)
        .unwrap_or_else(FrameRef::int)
}
