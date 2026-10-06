//! Gate 7.3 item 1: a hyperedge touches; it never crosses.

use joinn_frame::Verdict;
use joinn_gpu::open;
use joinn_visual::{Camera, GALAXY_TAG, LINK_TAG, Pick, SYSTEM_TAG, TAG_MASK, cpu_pick_at};

use super::g6_adapters::g6_adapters;
use super::g73_pictures::g73_pictures;
use super::grove::grove_layout;
use super::links::{LINK_UNIVERSES, fold_line, laid_universe};
use super::pick::grove_subjects;
use super::zoom::zoom_views;

/// On `universe.universe`, `ordered.universe` and the grove, every fold
/// state's routes line says `crossings 0` and no link touches a node twice
/// (§2.6). On every adapter, at every §2.12 view of the grove, no pixel the
/// CPU gives a body (its surface, cells, ports, wires or text) when links are
/// left out has a link as its GPU owner: links lie under bodies (§2.5).
pub(crate) fn g73_touch() -> bool {
    let mut failures = Vec::new();
    for arg in LINK_UNIVERSES {
        let (name, universe, layout) = match laid_universe(arg) {
            Ok(l) => l,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let all = &layout.routes;
        for fold in &all.folds {
            let of_fold: Vec<_> = all.routes.iter().filter(|r| r.fold == *fold).collect();
            let (line, found) = fold_line(&name, *fold, &layout, &universe.coding.links, &of_fold);
            println!("{line}");
            failures.extend(found);
        }
    }
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let layout = match grove_layout() {
        Ok((_, l)) => l,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let (scenes, views) = match grove_subjects(&layout, zoom_views(&layout)) {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut bodies: Vec<Vec<usize>> = Vec::new();
    for v in &views {
        let shapes = match scenes.get(v.scene).map(|s| s.shapes_at(&v.camera)) {
            Some(Verdict::Ok(s)) => s,
            Some(Verdict::Refused(r)) => {
                println!("{}", r.reason);
                return false;
            }
            None => {
                println!("gate 7.3: grove {}: no scene {}", v.label, v.scene);
                return false;
            }
        };
        let unlinked: Vec<_> = shapes
            .into_iter()
            .filter(|s| s.id[2] & TAG_MASK != LINK_TAG)
            .collect();
        let pick = match cpu_pick_at(&unlinked, &v.camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        bodies.push(
            (0..)
                .zip(&pick.pixels)
                .filter_map(|(i, p)| match p {
                    Pick::Owned(id) if !matches!(id[2] & TAG_MASK, SYSTEM_TAG | GALAXY_TAG) => {
                        Some(i)
                    }
                    _ => None,
                })
                .collect(),
        );
    }
    let cameras: Vec<(usize, Camera)> = views.iter().map(|v| (v.scene, v.camera)).collect();
    let body_pixels: usize = bodies.iter().map(Vec::len).sum();
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let images = match g73_pictures(&gpu, &scenes, &cameras) {
            Ok(i) => i,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let (mut linked, mut over) = (0usize, 0usize);
        for ((v, body), image) in views.iter().zip(&bodies).zip(&images) {
            let is_link = |id: &[u32; 4]| id[2] & TAG_MASK == LINK_TAG;
            linked += image.iter().filter(|id| is_link(id)).count();
            for &i in body {
                let Some(id) = image.get(i).filter(|id| is_link(id)) else {
                    continue;
                };
                over += 1;
                if over <= 5 {
                    let owner = scenes
                        .get(v.scene)
                        .map(|s| s.print_id(*id))
                        .and_then(|p| match p {
                            Verdict::Ok(name) => Some(name),
                            Verdict::Refused(_) => None,
                        })
                        .unwrap_or_else(|| format!("{id:?}"));
                    let w = v.camera.width as usize;
                    failures.push(format!(
                        "touch {}: grove {}: pixel {},{} of a body is owned by {owner} on the GPU; acceptance is the body",
                        adapter.line(),
                        v.label,
                        i % w,
                        i / w
                    ));
                }
            }
        }
        println!(
            "touch {}: {} views, link pixels {linked}, body pixels {body_pixels}, body pixels a link owns {over}",
            adapter.line(),
            views.len()
        );
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
