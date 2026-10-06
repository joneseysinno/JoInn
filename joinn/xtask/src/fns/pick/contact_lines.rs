//! The contact pictures on one adapter, at every standard viewport.

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, OFFSCREEN_FORMAT, Renderer};
use joinn_visual::{TAG_MASK, WIRE_TAG, shapes_of_tables};
use std::collections::BTreeSet;

use super::ContactSubject;
use super::compare::compare;
use super::owner_name::owner_name;
use crate::fns::once::say;

/// One line per contact and viewport: `<label> <w>x<h>: agree, edge,
/// disagree, owners n/total, links l`. A disagreement, a count that doesn't sum
/// to the image, an owner with no pixel, or any link owner (V130) fails.
pub(crate) fn contact_lines(
    gpu: &Gpu,
    adapter: &str,
    subjects: &[ContactSubject],
    failures: &mut Vec<String>,
) -> Result<(), String> {
    for (label, scene, picks) in subjects {
        let total = shapes_of_tables(scene.tables()).len();
        let mut renderer = match Renderer::new(gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.upload_all(gpu, scene.tables()) {
            return Err(r.reason);
        }
        for (camera, cpu) in picks {
            let picture = match renderer.picture(gpu, camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let t = compare(cpu, &picture.ids);
            let links: BTreeSet<[u32; 4]> = picture
                .ids
                .iter()
                .chain(t.owners.iter())
                .filter(|id| **id != [0; 4] && (id[2] & TAG_MASK) == WIRE_TAG)
                .copied()
                .collect();
            let name = format!("{label} {}x{}", camera.width, camera.height);
            say(&format!(
                "{name}: agree {}, edge {}, disagree {}, owners {}/{total}, links {}",
                t.agree,
                t.edge,
                t.disagree,
                t.owners.len(),
                links.len()
            ));
            let pixels = camera.width as usize * camera.height as usize;
            if t.agree + t.edge + t.disagree != pixels {
                failures.push(format!(
                    "{adapter}: {name}: agree + edge + disagree = {}, not {pixels}",
                    t.agree + t.edge + t.disagree
                ));
            }
            for (i, want, got) in &t.first {
                let (x, y) = (i % camera.width as usize, i / camera.width as usize);
                failures.push(format!(
                    "{adapter}: {name}: TRUTH VIOLATION at {x},{y}: cpu {}, gpu {}",
                    owner_name(scene, *want),
                    owner_name(scene, *got)
                ));
            }
            if t.owners.len() != total {
                failures.push(format!(
                    "{adapter}: {name}: {} of {total} owners own a non-edge pixel",
                    t.owners.len()
                ));
            }
            if !links.is_empty() {
                failures.push(format!(
                    "{adapter}: {name}: {} link owner(s) in a contact picture; a force owns no pixel (V130)",
                    links.len()
                ));
            }
        }
    }
    Ok(())
}
