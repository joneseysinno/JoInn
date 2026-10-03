//! One viewport: every non-edge GPU ID names `cpu_pick`'s owner, and all 13 do.

use std::collections::BTreeSet;

use joinn_frame::Verdict;
use joinn_visual::{FitCamera, Pick, PickImage, Scene};

/// Disagreeing pixels and a short owner set are errors. Edge pixels are counted
/// by being skipped: they are never judged.
pub(crate) fn g6_agree(
    scene: &Scene,
    camera: &FitCamera,
    cpu: &PickImage,
    gpu: &[[u32; 4]],
    adapter: &str,
) -> Result<(), String> {
    let pixels = camera.width as usize * camera.height as usize;
    if gpu.len() != pixels || cpu.pixels.len() != pixels {
        return Err(format!(
            "{adapter}: {}x{}: GPU image has {} id(s) and the CPU pick has {}; acceptance is {pixels}",
            camera.width,
            camera.height,
            gpu.len(),
            cpu.pixels.len()
        ));
    }
    let name = |id: [u32; 4]| -> String {
        match scene.resolve(id) {
            Verdict::Ok(owner) => scene.print_owner(&owner),
            Verdict::Refused(r) => format!("{id:?} ({})", r.reason),
        }
    };
    let mut owners = BTreeSet::new();
    for (i, (pick, id)) in cpu.pixels.iter().zip(gpu).enumerate() {
        let want = match pick {
            Pick::Edge => continue,
            Pick::Background => [0; 4],
            Pick::Owned(owner) => *owner,
        };
        if want != *id {
            let x = i % camera.width as usize;
            let y = i / camera.width as usize;
            return Err(format!(
                "{adapter}: {}x{}: TRUTH VIOLATION at {x},{y}: cpu {}, gpu {}",
                camera.width,
                camera.height,
                name(want),
                name(*id)
            ));
        }
        if want != [0; 4] {
            owners.insert(want);
        }
    }
    if owners.len() != 13 {
        return Err(format!(
            "{adapter}: {}x{}: {} of 13 owners own a non-edge pixel",
            camera.width,
            camera.height,
            owners.len()
        ));
    }
    Ok(())
}
