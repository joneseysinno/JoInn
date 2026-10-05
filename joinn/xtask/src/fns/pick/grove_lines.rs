//! The grove's views on one adapter.

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, OFFSCREEN_FORMAT, Renderer};
use joinn_visual::{Pick, UniverseScene};
use std::collections::BTreeSet;

use super::GroveView;
use super::compare::compare;

/// One line per view: `grove <view>: sample <n> agree <a>, edge <e>, disagree
/// <d>, owners <n> (cut allows <n>)`. The sample is judged against the grid
/// pick; the whole image is judged too, and its owners are the GPU's owners of
/// pixels the CPU doesn't call edge. A disagreement anywhere, a sampled pixel
/// the brute-force walk names differently, or owners that differ from the
/// cut's (V145) fails. The lines go to `lines`, the failures to `failures`.
pub(crate) fn grove_lines(
    gpu: &Gpu,
    adapter: &str,
    (scenes, views): (&[UniverseScene], &[GroveView]),
    lines: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Result<(), String> {
    let mut renderers: Vec<Renderer> = Vec::new();
    for scene in scenes {
        let mut r = match Renderer::new(gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = r.upload_all(gpu, scene.tables()) {
            return Err(r.reason);
        }
        renderers.push(r);
    }
    for v in views {
        let Some(renderer) = renderers.get_mut(v.scene) else {
            return Err(format!("pick: grove {}: no scene {}", v.label, v.scene));
        };
        let picture = match renderer.picture_at(gpu, &v.camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let (mut agree, mut edge, mut disagree) = (0usize, 0usize, 0usize);
        for &i in &v.sample {
            match (v.cpu.pixels.get(i), picture.ids.get(i)) {
                (Some(Pick::Edge), _) => edge += 1,
                (Some(Pick::Background), Some(id)) if *id == [0; 4] => agree += 1,
                (Some(Pick::Owned(want)), Some(id)) if want == id => agree += 1,
                _ => disagree += 1,
            }
        }
        let whole = compare(&v.cpu, &picture.ids);
        let owners: BTreeSet<[u32; 4]> = v
            .cpu
            .pixels
            .iter()
            .zip(&picture.ids)
            .filter(|(p, id)| **p != Pick::Edge && **id != [0; 4])
            .map(|(_, id)| *id)
            .collect();
        lines.push(format!(
            "grove {}: sample {} agree {agree}, edge {edge}, disagree {disagree}, owners {} (cut allows {})",
            v.label,
            v.sample.len(),
            owners.len(),
            v.allows.len()
        ));
        let line = format!("{adapter}: grove {}", v.label);
        if v.differ != 0 {
            failures.push(format!(
                "{line}: cpu_pick and cpu_pick_reference name {} sampled pixel(s) differently",
                v.differ
            ));
        }
        for (i, cpu, got) in &whole.first {
            let (x, y) = (i % v.camera.width as usize, i / v.camera.width as usize);
            failures.push(format!(
                "{line}: TRUTH VIOLATION at {x},{y}: cpu {cpu:?}, gpu {got:?}"
            ));
        }
        if whole.disagree != 0 {
            failures.push(format!(
                "{line}: {} pixel(s) of the whole image disagree",
                whole.disagree
            ));
        }
        if owners != v.allows {
            failures.push(format!(
                "{line}: owners in the GPU image {:?} differ from the cut's: only GPU {:?}, only cut {:?}",
                owners.len(),
                owners.difference(&v.allows).take(5).collect::<Vec<_>>(),
                v.allows.difference(&owners).take(5).collect::<Vec<_>>()
            ));
        }
    }
    Ok(())
}
