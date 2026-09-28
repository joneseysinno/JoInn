//! Refused bits a run cleared on instances it did not touch (Amendment A4).

use joinn_frame::Verdict;
use joinn_visual::{Owner, REFUSED, Scene, Tables};

/// Cells whose `refused` bit was set in `before` and is clear now, whose
/// instance is not in `touched`. Each adds one row to the V121 bound.
pub(super) fn cleared_elsewhere(scene: &Scene, before: &Tables, touched: &[String]) -> usize {
    (0u32..)
        .zip(scene.tables().cell.iter().zip(&before.cell))
        .filter(|(_, (now, was))| was.flags & REFUSED != 0 && now.flags & REFUSED == 0)
        .filter_map(|(slot, (now, _))| {
            match scene.resolve([now.body + 1, slot + 1, 0, now.generation]) {
                Verdict::Ok(Owner::Cell(name)) => Some(name),
                _ => None,
            }
        })
        .filter(|name| !touched.contains(name))
        .count()
}
