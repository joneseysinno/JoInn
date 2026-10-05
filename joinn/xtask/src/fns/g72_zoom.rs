//! Gate 7.2 item 1: the zoom is exact, and a rebase moves nothing.

use joinn_frame::Verdict;
use joinn_visual::{Camera, STANDARD_VIEWPORTS, Scene, fit};

use super::g6_adapters::g6_adapters;
use super::g72_refusals::g72_refusals;
use super::g72_reversible::g72_reversible;
use super::grove::grove_layout;
use super::regrow::grove_pass;
use super::{corpus_bodies, g6_owners, g6_picture, g7_picture};

/// §2.2's refusals word for word; the zoom script on the grove (pans and zooms
/// write no row, rebases write chart rows only, the end equals a fresh grow,
/// and every rebase draws identical bytes on every adapter); 8 notches in and
/// out about one pixel return the camera and the bytes (V144); the
/// calculator's Phase 6 cameras convert to their whole `k`; and gate 6's and
/// gate 7's picture items pass through the converted cameras.
pub(crate) fn g72_zoom() -> bool {
    let Some(all) = g6_adapters() else {
        return false;
    };
    let mut failures = g72_refusals();
    let mut lines = Vec::new();
    let run = grove_layout()
        .and_then(|(_, layout)| {
            grove_pass(&all, &mut lines, &mut failures)?;
            g72_reversible(&layout, &all)
        })
        .map(|more| failures.extend(more));
    if let Err(e) = run {
        println!("{e}");
        return false;
    }
    let rel = "phase2/calculator.body";
    let calculator = match corpus_bodies() {
        Ok(all) => all.into_iter().find(|(r, _)| r == rel),
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let scene = match calculator {
        Some((_, Verdict::Ok((body, cells)))) => Scene::grow("body", &body, &cells),
        Some((_, Verdict::Refused(r))) => Verdict::Refused(r),
        None => {
            println!("{rel} is not in the corpus");
            return false;
        }
    };
    let scene = match scene {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => {
            println!("{rel}: {}", r.reason);
            return false;
        }
    };
    for (w, h) in STANDARD_VIEWPORTS {
        let f = fit(scene.layout(), w, h);
        match Camera::from_fit(&f) {
            Verdict::Ok(c) if c.zoom.whole_k() == Some(f.k) && c.pin == (f.ox, f.oy) => {}
            Verdict::Ok(c) => failures.push(format!(
                "calculator {w}x{h}: k {} converts to {c:?}; acceptance is the same k and pin",
                f.k
            )),
            Verdict::Refused(r) => failures.push(format!("calculator {w}x{h}: {}", r.reason)),
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty() && g6_owners() && g6_picture() && g7_picture()
}
