//! The plant (rule 19): a CPU pick of tables the GPU didn't draw.

use joinn_frame::Verdict;
use joinn_visual::{Camera, Scene, cpu_pick, shapes_of_tables};

use super::compare::compare;
use super::port_slot::port_slot;

/// `cpu_pick` on a copy of the scene's tables with port `sum@1`'s centre moved
/// down one layout unit, against the GPU's IDs of the unmoved tables. Returns
/// the number of pixels that disagree.
pub(super) fn plant(scene: &Scene, camera: &Camera, gpu: &[[u32; 4]]) -> Result<usize, String> {
    let printed = format!("{}.sum@1", scene.alias());
    let slot = port_slot(scene, &printed)
        .ok_or_else(|| format!("pick plant: no port {printed} in the calculator's tables"))?;
    let mut moved = scene.tables().clone();
    let row = moved
        .port
        .get_mut(slot as usize)
        .ok_or_else(|| format!("pick plant: port slot {slot} is past the table"))?;
    row.y += 1;
    match cpu_pick(&shapes_of_tables(&moved), camera) {
        Verdict::Ok(cpu) => Ok(compare(&cpu, gpu).disagree),
        Verdict::Refused(r) => Err(format!("pick plant: {}", r.reason)),
    }
}
