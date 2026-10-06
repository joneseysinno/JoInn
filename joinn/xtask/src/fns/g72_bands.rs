//! Gate 7.2 item 2: bands follow size, and two pickers name every pixel.

use joinn_frame::Verdict;
use joinn_gpu::open;
use joinn_visual::{CutForm, THRESHOLDS, cut, owner_band};

use super::g6_adapters::g6_adapters;
use super::grove::grove_layout;
use super::pick::{band_boundary, grove_lines, grove_subjects};
use super::zoom::{ZOOM_BLOCK, fade_views, zoom_text, zoom_views};

/// The CPU cut prints §2.12's block; each threshold has a fade-window view
/// whose body the cut draws in its owner band, fading; and on every adapter,
/// at every §2.12 view and every fade view, `cpu_pick` equals
/// `cpu_pick_reference` on the seeded pixels, every non-edge pixel's GPU owner
/// equals `cpu_pick`'s, and the GPU image's owners are the cut's (V145); and
/// the shader's `band()` at the 240 px boundary equals `owner_band`'s.
pub(crate) fn g72_bands() -> bool {
    let Some(all) = g6_adapters() else {
        return false;
    };
    let layout = match grove_layout() {
        Ok((_, l)) => l,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    match zoom_text() {
        Ok(text) if text == ZOOM_BLOCK => {}
        Ok(text) => failures.push(format!(
            "the cut's counts differ from §2.12's lines:\n{text}"
        )),
        Err(e) => failures.push(e),
    }
    let fades = fade_views(&layout);
    if fades.len() != THRESHOLDS.len() {
        failures.push(format!(
            "{} fade-window view(s); acceptance is one per threshold, {}",
            fades.len(),
            THRESHOLDS.len()
        ));
    }
    for (label, camera, body) in &fades {
        let size = layout.chart(*body).map_or(0, |c| c.size.0.max(c.size.1));
        let want = CutForm::Drawn(owner_band(camera.zoom, size));
        let c = cut(&layout, camera);
        match c.entries.iter().find(|e| e.chart == *body) {
            Some(e) if e.form == want && e.fading => {}
            got => failures.push(format!(
                "{label}: the cut has {got:?}; acceptance is {want:?}, fading"
            )),
        }
    }
    let cameras = zoom_views(&layout)
        .into_iter()
        .chain(fades.into_iter().map(|(l, c, _)| (l, c)))
        .collect();
    let (scenes, views) = match grove_subjects(&layout, cameras) {
        Ok(s) => s,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut lines = Vec::new();
    for adapter in &all {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let judged = grove_lines(
            &gpu,
            adapter.line(),
            (&scenes, &views),
            &mut lines,
            &mut failures,
        );
        if let Err(e) = judged {
            println!("{e}");
            return false;
        }
        match band_boundary(&gpu, adapter.line()) {
            Ok((line, missed)) => {
                println!("{line}");
                failures.extend(missed);
            }
            Err(e) => {
                println!("{e}");
                return false;
            }
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
