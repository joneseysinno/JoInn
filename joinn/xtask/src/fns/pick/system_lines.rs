//! Plan 7.4's systems on one adapter.

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, OFFSCREEN_FORMAT, Renderer};
use joinn_visual::{FORCE_TAG, Pick, TAG_MASK};
use std::collections::BTreeSet;

use super::SystemSubject;
use super::compare::compare;

/// One line per system and size: `system <name> n <k>: agree <a>, edge <e>,
/// disagree <d>, owners <n> (cut allows <n>), force owners <f> (cut allows
/// <f>)`. Owners are the GPU's owners of pixels the CPU doesn't call edge,
/// the force counted apart (V164). A disagreement, or owners other than the
/// system's cut, fails. The lines go to `lines`, the failures to `failures`.
pub(crate) fn system_lines(
    gpu: &Gpu,
    adapter: &str,
    subjects: &[SystemSubject],
    lines: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Result<(), String> {
    for (label, scene, camera, cpu) in subjects {
        let mut renderer = match Renderer::new(gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.upload_all(gpu, scene.tables()) {
            return Err(r.reason);
        }
        let picture = match renderer.picture(gpu, camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let t = compare(cpu, &picture.ids);
        let owners: BTreeSet<[u32; 4]> = cpu
            .pixels
            .iter()
            .zip(&picture.ids)
            .filter(|(p, id)| **p != Pick::Edge && **id != [0; 4])
            .map(|(_, id)| *id)
            .collect();
        let allows = scene.allows();
        let is_force = |id: &&[u32; 4]| id[2] & TAG_MASK == FORCE_TAG;
        let forces = owners.iter().filter(is_force).count();
        let allowed_forces = allows.iter().filter(is_force).count();
        lines.push(format!(
            "{label}: agree {}, edge {}, disagree {}, owners {} (cut allows {}), force owners {forces} (cut allows {allowed_forces})",
            t.agree,
            t.edge,
            t.disagree,
            owners.len() - forces,
            allows.len() - allowed_forces
        ));
        let line = format!("{adapter}: {label}");
        let pixels = camera.width as usize * camera.height as usize;
        if t.agree + t.edge + t.disagree != pixels {
            failures.push(format!(
                "{line}: agree + edge + disagree = {}, not {pixels}",
                t.agree + t.edge + t.disagree
            ));
        }
        for (i, want, got) in &t.first {
            let (x, y) = (i % camera.width as usize, i / camera.width as usize);
            failures.push(format!(
                "{line}: TRUTH VIOLATION at {x},{y}: cpu {}, gpu {}",
                scene.print_id(*want),
                scene.print_id(*got)
            ));
        }
        if owners != allows {
            failures.push(format!(
                "{line}: owners in the GPU image differ from the cut's: only GPU {:?}, only cut {:?}",
                owners
                    .difference(&allows)
                    .map(|id| scene.print_id(*id))
                    .collect::<Vec<_>>(),
                allows
                    .difference(&owners)
                    .map(|id| scene.print_id(*id))
                    .collect::<Vec<_>>()
            ));
        }
    }
    Ok(())
}
