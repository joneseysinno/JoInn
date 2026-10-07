//! Gate 7.4 item 3: a force is seen as a lasso; growth is not identity.

use joinn_dna::hash;
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Value, Verdict};
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open};
use joinn_link::grow;
use joinn_visual::{FORCE_TAG, Pick, TAG_MASK, check_lasso, cpu_pick, layout_system};

use super::forces::corpus_cells;
use super::g6_adapters::g6_adapters;
use super::grow::{GROW_SYSTEMS, corpus_system, seven_inputs};
use super::pick::system_subjects;

/// The sizes the lasso's rules are checked at: both row wraps.
const LAST: usize = 13;

/// Each system grown by its seven inputs, repeated, at sizes 0 … 13: §2.8's
/// lasso rules hold and `hash_system` is the golden hash at every size. On
/// every adapter, at sizes 0, 3 and 7 of both systems: the GPU agrees with the
/// CPU on every pixel, no pixel the CPU gives a body, cell, port or text when
/// the lasso is left out is owned by the force (the lasso lies under them), and
/// a force pixel names `force count`.
pub(crate) fn g74_lasso() -> bool {
    let frames = FrameRegistry::phase1();
    let cells = match corpus_cells(&frames) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    for name in GROW_SYSTEMS {
        let s = match corpus_system(name) {
            Ok(s) => s,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let values: Result<Vec<Value>, String> = seven_inputs(s.accepts)
            .iter()
            .cycle()
            .take(LAST)
            .map(|v| match IntFrame::new().canonicalize(Term::int(*v)) {
                Verdict::Ok(v) => Ok(v),
                Verdict::Refused(r) => Err(r.reason),
            })
            .collect();
        let values = match values {
            Ok(v) => v,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let before = failures.len();
        for n in 0..=LAST {
            let grown = match grow(
                &s.system,
                &s.contacts,
                &cells,
                values.get(..n).unwrap_or(&[]),
            ) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => {
                    failures.push(format!("lasso {name} n {n}: {}", r.reason));
                    continue;
                }
            };
            let h = hash(&grown.system().coding).to_hex();
            if h != s.golden {
                failures.push(format!(
                    "lasso {name} n {n}: hash {h} is not the system's {}",
                    s.golden
                ));
            }
            let checked = match layout_system(&s.system, &s.contacts, &cells, &grown, s.waiting) {
                Verdict::Ok(l) => check_lasso(&l),
                Verdict::Refused(r) => Verdict::Refused(r),
            };
            if let Verdict::Refused(r) = checked {
                failures.push(format!("lasso {name} n {n}: {}", r.reason));
            }
        }
        if failures.len() == before {
            println!(
                "lasso {name}: rules hold at sizes 0 … {LAST}; hash {} at every size",
                &s.golden[..12.min(s.golden.len())]
            );
        }
    }
    let subjects = match system_subjects() {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let is_force = |id: &[u32; 4]| id[2] & TAG_MASK == FORCE_TAG;
    let mut under = Vec::new();
    for (label, scene, camera, _) in &subjects {
        let shapes: Vec<_> = scene
            .shapes()
            .into_iter()
            .filter(|s| !is_force(&s.id))
            .collect();
        match cpu_pick(&shapes, camera) {
            Verdict::Ok(p) => under.push(
                (0..)
                    .zip(&p.pixels)
                    .filter_map(|(i, p)| matches!(p, Pick::Owned(_)).then_some(i))
                    .collect::<Vec<usize>>(),
            ),
            Verdict::Refused(r) => {
                println!("lasso {label}: {}", r.reason);
                return false;
            }
        }
    }
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let (mut disagree, mut force_pixels, mut covered, mut named) =
            (0usize, 0usize, 0usize, true);
        for ((label, scene, camera, cpu), body) in subjects.iter().zip(&under) {
            let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            if let Verdict::Refused(r) = renderer.upload_all(&gpu, scene.tables()) {
                println!("{}", r.reason);
                return false;
            }
            let ids = match renderer.picture(&gpu, camera) {
                Verdict::Ok(p) => p.ids,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            let w = camera.width as usize;
            for (i, (want, got)) in cpu.pixels.iter().zip(&ids).enumerate() {
                let same = match want {
                    Pick::Edge => true,
                    Pick::Background => *got == [0; 4],
                    Pick::Owned(id) => id == got,
                };
                if !same {
                    disagree += 1;
                    if disagree <= 5 {
                        failures.push(format!(
                            "lasso {}: {label}: pixel {},{}: cpu {want:?}, gpu {}",
                            adapter.line(),
                            i % w,
                            i / w,
                            scene.print_id(*got)
                        ));
                    }
                }
            }
            let forced: Vec<usize> = (0..)
                .zip(&ids)
                .filter_map(|(i, id)| is_force(id).then_some(i))
                .collect();
            force_pixels += forced.len();
            match forced.first().and_then(|i| ids.get(*i)) {
                Some(id) if scene.print_id(*id) == "force count" => {}
                other => {
                    named = false;
                    failures.push(format!(
                        "lasso {}: {label}: a force pixel names {:?}; acceptance is force count",
                        adapter.line(),
                        other.map(|id| scene.print_id(*id))
                    ));
                }
            }
            for &i in body {
                if ids.get(i).is_some_and(is_force) {
                    covered += 1;
                    if covered <= 5 {
                        let owner = match cpu.pixels.get(i) {
                            Some(Pick::Owned(id)) => scene.print_id(*id),
                            _ => "a body".to_owned(),
                        };
                        failures.push(format!(
                            "lasso {}: {label}: pixel {},{} of {owner} is owned by the force on the GPU; acceptance is {owner}",
                            adapter.line(),
                            i % w,
                            i / w
                        ));
                    }
                }
            }
        }
        println!(
            "lasso {}: {} pictures, disagree {disagree}, force pixels {force_pixels}, body pixels the force owns {covered}; a force pixel names {}",
            adapter.line(),
            subjects.len(),
            if named {
                "force count"
            } else {
                "something else"
            }
        );
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
