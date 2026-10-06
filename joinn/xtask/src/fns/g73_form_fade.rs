//! Gate 7.3 item 2: in a form crossfade window, a link's pixels are its owner
//! form's.

use std::collections::{BTreeMap, BTreeSet};

use joinn_frame::Verdict;
use joinn_gpu::{GpuAdapter, open};
use joinn_visual::{
    Camera, FORM_THRESHOLDS, Form, LINK_TAG, Pick, TAG_MASK, UniverseLayout, fold_at, form_of,
};

use super::g73_pictures::g73_pictures;
use super::pick::grove_subjects;
use super::zoom::zoom_views;

/// Per form threshold, of every route in that threshold's window at a §2.12
/// view of the grove (in the view's fold state), the one that owns the most
/// pixels of that view's `cpu_pick` image, the earlier view on a tie. Every
/// pixel of its link, in the CPU image and in the GPU's on every adapter,
/// carries its owner form, and the GPU's image has some. The lines (one per
/// threshold, then one per threshold per adapter) and the failures.
pub(crate) fn g73_form_fade(
    layout: &UniverseLayout,
    adapters: &[GpuAdapter],
) -> Result<(Vec<String>, Vec<String>), String> {
    let (scenes, views) = grove_subjects(layout, zoom_views(layout))?;
    let forms = [Form::Region, Form::Hub, Form::Bundle, Form::Spine];
    let name = |n: u32| forms.get(n as usize).map_or("no form", |f| f.name());
    let link_of =
        |id: &[u32; 4]| (id[2] & TAG_MASK == LINK_TAG).then(|| (id[0], id[2] & !TAG_MASK));
    let mut lines = Vec::new();
    let mut failures = Vec::new();
    let mut chosen: Vec<(i64, usize, u32, Form)> = Vec::new();
    for t in FORM_THRESHOLDS {
        let found = views.iter().enumerate().filter_map(|(i, v)| {
            let mut owned: BTreeMap<u32, (usize, BTreeSet<u32>)> = BTreeMap::new();
            for p in &v.cpu.pixels {
                if let Some((slot, form)) = match p {
                    Pick::Owned(id) => link_of(id),
                    _ => None,
                } {
                    let entry = owned.entry(slot).or_default();
                    entry.0 += 1;
                    entry.1.insert(form);
                }
            }
            let fold = fold_at(v.camera.zoom);
            layout
                .routes
                .routes
                .iter()
                .filter(|r| r.fold == fold && form_of(v.camera.zoom, r).1 == Some(t))
                .filter_map(|r| {
                    let slot = u32::try_from(r.link + 1).ok()?;
                    let (n, seen) = owned.get(&slot)?;
                    Some((*n, slot, seen.clone(), r))
                })
                .max_by_key(|(n, slot, _, _)| (*n, std::cmp::Reverse(*slot)))
                .map(|(n, slot, seen, r)| (i, n, slot, seen, r))
        });
        let found = found.max_by_key(|(i, n, _, _, _)| (*n, std::cmp::Reverse(*i)));
        let Some((i, n, slot, seen, route)) = found else {
            failures.push(format!(
                "form fade {t}: no §2.12 view has a link in the window that owns a pixel"
            ));
            continue;
        };
        let Some(v) = views.get(i) else {
            continue;
        };
        let owner = form_of(v.camera.zoom, route).0;
        let id = layout
            .routes
            .ids
            .get(route.link)
            .map_or("?", String::as_str);
        let every: Vec<&str> = seen.iter().map(|f| name(*f)).collect();
        lines.push(format!(
            "form fade {t}: grove {}, link {id} (size {}) owner {}: cpu {n} pixels, forms {}",
            v.label,
            route.size,
            owner.name(),
            every.join(", ")
        ));
        if seen != BTreeSet::from([owner.number()]) {
            failures.push(format!(
                "form fade {t}: link {id} at {}: cpu forms {}; acceptance is {} only",
                v.label,
                every.join(", "),
                owner.name()
            ));
        }
        chosen.push((t, i, slot, owner));
    }
    let cameras: Vec<(usize, Camera)> = chosen
        .iter()
        .filter_map(|(_, i, _, _)| views.get(*i).map(|v| (v.scene, v.camera)))
        .collect();
    for adapter in adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let images = g73_pictures(&gpu, &scenes, &cameras)?;
        for ((t, _, slot, owner), image) in chosen.iter().zip(&images) {
            let mut n = 0usize;
            let mut seen = BTreeSet::new();
            for (s, form) in image.iter().filter_map(link_of) {
                if s == *slot {
                    n += 1;
                    seen.insert(form);
                }
            }
            let every: Vec<&str> = seen.iter().map(|f| name(*f)).collect();
            lines.push(format!(
                "form fade {t} {}: gpu {n} pixels, forms {}",
                adapter.line(),
                every.join(", ")
            ));
            if n == 0 || seen != BTreeSet::from([owner.number()]) {
                failures.push(format!(
                    "form fade {t} {}: gpu {n} pixels, forms {}; acceptance is some, {} only",
                    adapter.line(),
                    every.join(", "),
                    owner.name()
                ));
            }
        }
    }
    Ok((lines, failures))
}
