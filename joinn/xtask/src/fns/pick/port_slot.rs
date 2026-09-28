//! Find a port's slot by the owner its ID prints as.

use joinn_frame::Verdict;
use joinn_visual::{PORT_TAG, Scene};

/// The slot whose port ID resolves to `printed` (for example `body.sum@1`).
pub(crate) fn port_slot(scene: &Scene, printed: &str) -> Option<u32> {
    let t = scene.tables();
    (0u32..).zip(&t.port).find_map(|(slot, p)| {
        let cell = t.cell.get(p.cell as usize)?;
        let id = [
            cell.body + 1,
            p.cell + 1,
            PORT_TAG | p.position,
            cell.generation,
        ];
        match scene.resolve(id) {
            Verdict::Ok(o) if scene.print_owner(&o) == printed => Some(slot),
            _ => None,
        }
    })
}
