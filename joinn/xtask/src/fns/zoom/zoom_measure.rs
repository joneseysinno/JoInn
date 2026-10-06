//! `cargo xtask zoom --measure`: what the cut costs per view (information only).

use joinn_visual::{ChartId, UniverseScene, cut, print_zoom};
use std::fmt::Write;
use std::time::{Duration, Instant};

use super::zoom_views;
use crate::fns::grove::grove_layout;

/// Runs of the CPU cut per view; the least is printed.
const RUNS: usize = 5;

/// Per §2.12 view, `measure <view> <zoom>: cut cpu <µs> µs, draw instances <n>`:
/// the least of five CPU cuts, and the instances one GPU draw issues (every
/// row of every drawn kind, through the owner and the ghost pipelines: frame
/// rows twice, as open frames and as lens nodes, then segments, bodies twice,
/// cells, wires, ports, strokes; the shader decides per instance, so this does
/// not depend on the view). Wall
/// time is printed and decides nothing (rule 4).
// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn zoom_measure() -> Result<String, String> {
    let (_, layout) = grove_layout()?;
    let scene = match UniverseScene::grow(layout.clone(), ChartId(0)) {
        joinn_frame::Verdict::Ok(s) => s,
        joinn_frame::Verdict::Refused(r) => return Err(r.reason),
    };
    let t = scene.tables();
    let per_pass = 2 * t.frame.len()
        + t.segment.len()
        + 2 * t.body.len()
        + t.cell.len()
        + t.link.len()
        + t.port.len()
        + t.stroke.len();
    let instances = 2 * per_pass;
    let mut out = String::new();
    for (label, camera) in zoom_views(&layout) {
        let mut least = Duration::MAX;
        let mut entries = 0;
        for _ in 0..RUNS {
            let start = Instant::now();
            let c = cut(&layout, &camera);
            least = least.min(start.elapsed());
            entries = c.entries.len();
        }
        let _ = writeln!(
            out,
            "measure {label} {}: cut cpu {} µs ({entries} entries), draw instances {instances}",
            print_zoom(&camera.zoom),
            least.as_micros()
        );
    }
    let _ = writeln!(
        out,
        "zoom --measure: {} views; tables frame {}, body {}, cell {}, link {}, port {}, stroke {}, route {}, segment {}",
        zoom_views(&layout).len(),
        t.frame.len(),
        t.body.len(),
        t.cell.len(),
        t.link.len(),
        t.port.len(),
        t.stroke.len(),
        t.route.len(),
        t.segment.len()
    );
    Ok(out)
}
