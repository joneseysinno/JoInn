//! The grove's views and their CPU picks, made once for every adapter.

use joinn_frame::Verdict;
use joinn_visual::{Pick, UniverseScene, cpu_pick_at, cpu_pick_sample};

use super::{GROVE_SAMPLE, GroveView};
use crate::fns::grove::{GROVE_SEED, grove_layout, splitmix64};
use crate::fns::zoom::zoom_views;

/// One scene per anchor the plan 7.2 2.12 views rebase onto, and per view the
/// grid pick of the whole image, the sampled pixels (SplitMix64 from the
/// grove's seed: x from the low half of a draw, y from the high half), how
/// many of them the brute-force walk names differently, and the owners the
/// cut's shapes give a pixel.
pub(crate) fn grove_subjects() -> Result<(Vec<UniverseScene>, Vec<GroveView>), String> {
    let (_, layout) = grove_layout()?;
    let mut scenes: Vec<UniverseScene> = Vec::new();
    let mut views = Vec::new();
    for (label, camera) in zoom_views(&layout) {
        let scene = match scenes.iter().position(|s| s.anchor() == camera.anchor) {
            Some(i) => i,
            None => match UniverseScene::grow(layout.clone(), camera.anchor) {
                Verdict::Ok(s) => {
                    scenes.push(s);
                    scenes.len() - 1
                }
                Verdict::Refused(r) => return Err(format!("pick: grove: {}", r.reason)),
            },
        };
        let shapes = match scenes[scene].shapes_at(&camera) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(format!("pick: grove: {}", r.reason)),
        };
        let cpu = match cpu_pick_at(&shapes, &camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(format!("pick: grove: {}", r.reason)),
        };
        let mut state = GROVE_SEED;
        let pixels: Vec<(u32, u32)> = (0..GROVE_SAMPLE)
            .map(|_| {
                let z = splitmix64(&mut state);
                (
                    (z as u32) % camera.width,
                    ((z >> 32) as u32) % camera.height,
                )
            })
            .collect();
        let reference = match cpu_pick_sample(&shapes, &camera, &pixels) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(format!("pick: grove: {}", r.reason)),
        };
        let sample: Vec<usize> = pixels
            .iter()
            .map(|&(x, y)| y as usize * camera.width as usize + x as usize)
            .collect();
        let differ = sample
            .iter()
            .zip(&reference)
            .filter(|(i, r)| cpu.pixels.get(**i) != Some(*r))
            .count();
        let allows = cpu
            .pixels
            .iter()
            .filter_map(|p| match p {
                Pick::Owned(id) => Some(*id),
                _ => None,
            })
            .collect();
        let label = if label == "frame" {
            label
        } else {
            format!("s level {} step {}", camera.zoom.level, camera.zoom.step)
        };
        views.push(GroveView {
            label,
            camera,
            scene,
            cpu,
            sample,
            differ,
            allows,
        });
    }
    Ok((scenes, views))
}
