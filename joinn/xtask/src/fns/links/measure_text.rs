//! `cargo xtask links --measure`: what routing costs at grow (information only).

use joinn_frame::Verdict;
use joinn_visual::routes;
use std::fmt::Write;
use std::time::{Duration, Instant};

use super::{LINK_UNIVERSES, laid_universe};

/// Runs of `routes` per universe; the least is printed.
const RUNS: usize = 5;

/// Per universe, `measure <name>: route µs <t> (<l> links, <r> routes, <p>
/// pieces; per link <t/l> µs)`: the least of five runs of `routes` over the
/// laid-out universe (every fold state's grid, graph, knots, legs, stubs and
/// arrowheads). Routing runs once per grow, never per tick. Wall time is
/// printed and decides nothing (rule 4); a release build is advised.
// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn measure_text() -> Result<String, String> {
    let mut out = String::new();
    for arg in LINK_UNIVERSES {
        let (name, universe, layout) = laid_universe(arg)?;
        let links = &universe.coding.links;
        let mut least = Duration::MAX;
        let mut counts = (0, 0);
        for _ in 0..RUNS {
            let start = Instant::now();
            let all = match routes(&layout, links) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
            };
            least = least.min(start.elapsed());
            counts = (
                all.routes.len(),
                all.routes.iter().map(|r| r.pieces.len()).sum::<usize>(),
            );
        }
        let us = least.as_micros();
        let per_link = us / links.len().max(1) as u128;
        let _ = writeln!(
            out,
            "measure {name}: route µs {us} ({} links, {} routes, {} pieces; per link {per_link} µs)",
            links.len(),
            counts.0,
            counts.1
        );
    }
    let _ = writeln!(
        out,
        "links --measure: {} universes, least of {RUNS} runs (information, not a check)",
        LINK_UNIVERSES.len()
    );
    Ok(out)
}
