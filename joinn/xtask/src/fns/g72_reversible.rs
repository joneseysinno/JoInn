//! Gate 7.2 item 1: zoom about one pixel is reversible (V144).

use joinn_frame::Verdict;
use joinn_gpu::{GpuAdapter, OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{Camera, ChartId, Rect, UNIVERSE_SIZE, UniverseLayout, UniverseScene, Zoom};

use super::zoom::VIEWPORT;

/// The pixel the notches are taken about: not the frame's pin, so the first
/// notch refocuses.
const ABOUT: (i64, i64) = (700, 300);

/// On the grove framed and refocused at `ABOUT`: 8 notches in double `k`
/// (one level up, same step), and 8 more out return the identical camera; on
/// every adapter the two pictures' color and ID bytes are identical. One line
/// per failure.
pub(crate) fn g72_reversible(
    layout: &UniverseLayout,
    all: &[GpuAdapter],
) -> Result<Vec<String>, String> {
    let (w, h) = VIEWPORT;
    let universe = Rect {
        x: 0,
        y: 0,
        w: UNIVERSE_SIZE.0,
        h: UNIVERSE_SIZE.1,
    };
    let start = match Camera::frame(universe, ChartId(0), w, h).refocus(ABOUT) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut failures = Vec::new();
    let mut camera = start;
    for i in 0..16 {
        camera = match camera.zoom_about(ABOUT, if i < 8 { 1 } else { -1 }) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let doubled = Zoom {
            level: start.zoom.level + 1,
            ..start.zoom
        };
        if i == 7
            && camera
                != (Camera {
                    zoom: doubled,
                    ..start
                })
        {
            failures.push(format!(
                "8 notches in about {ABOUT:?}: {camera:?}; acceptance is {start:?} at {doubled:?}"
            ));
        }
    }
    if camera != start {
        failures.push(format!(
            "8 notches in and 8 out about {ABOUT:?}: {camera:?}; acceptance is {start:?} (V144)"
        ));
    }
    let scene = match UniverseScene::grow(layout.clone(), start.anchor) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    for adapter in all {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.upload_all(&gpu, scene.tables()) {
            return Err(r.reason);
        }
        let mut pictures = Vec::new();
        for c in [start, camera] {
            match renderer.picture_at(&gpu, &c) {
                Verdict::Ok(p) => pictures.push(p),
                Verdict::Refused(r) => return Err(r.reason),
            }
        }
        if let [a, b] = pictures.as_slice() {
            if a.color != b.color || a.ids != b.ids {
                failures.push(format!(
                    "{}: 8 notches in and 8 out drew different bytes (V144)",
                    adapter.line()
                ));
            }
        }
    }
    Ok(failures)
}
