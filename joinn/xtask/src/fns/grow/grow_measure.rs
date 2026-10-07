//! `cargo xtask grow --measure`: what one growth step costs as a body grows
//! (information only; plan 7.4 §7's adversary).

use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Value, Verdict};
use joinn_link::{grow_step, respond};
use joinn_visual::SystemScene;
use std::fmt::Write;
use std::time::{Duration, Instant};

use super::{GROW_SYSTEMS, corpus_system, seven_inputs};
use crate::fns::forces::corpus_cells;

/// Runs per size; the least is printed.
const RUNS: usize = 5;
/// The sizes a step is measured from.
const SIZES: [usize; 4] = [0, 6, 24, 96];

/// Per system, `grow <name> µs per step at n = 0, 6, 24, 96: <t…> (engine
/// <e…>)`: the least of five runs of one step from size n, the scene's
/// (`apply_growth`: engine, layout, rows) and the engine's alone (`grow_step`
/// then `respond`). The body is grown by its seven inputs, repeated. Wall time
/// decides nothing (rule 4); a release build is advised.
// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(crate) fn grow_measure() -> Result<String, String> {
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames)?;
    let mut out = String::new();
    for name in GROW_SYSTEMS {
        let s = corpus_system(name)?;
        let last = SIZES.iter().max().copied().unwrap_or(0);
        let values: Vec<Value> = seven_inputs(s.accepts)
            .iter()
            .cycle()
            .take(last + 1)
            .map(|v| match IntFrame::new().canonicalize(Term::int(*v)) {
                Verdict::Ok(v) => Ok(v),
                Verdict::Refused(r) => Err(r.reason),
            })
            .collect::<Result<_, _>>()?;
        let (mut steps, mut engines) = (Vec::new(), Vec::new());
        for n in SIZES {
            let (Some(before), Some(next)) = (values.get(..n), values.get(n)) else {
                return Err(format!("grow --measure {name}: no input {n}"));
            };
            let scene = match SystemScene::regrow(&s.system, &s.contacts, &cells, s.waiting, before)
            {
                Verdict::Ok(sc) => sc,
                Verdict::Refused(r) => return Err(format!("grow --measure {name}: {}", r.reason)),
            };
            let (mut step, mut engine) = (Duration::MAX, Duration::MAX);
            for _ in 0..RUNS {
                let mut grown = scene.clone();
                let start = Instant::now();
                if let Verdict::Refused(r) = grown.apply_growth(next) {
                    return Err(format!("grow --measure {name} n {n}: {}", r.reason));
                }
                step = step.min(start.elapsed());
                let start = Instant::now();
                let g = match grow_step(scene.grown(), next) {
                    Verdict::Ok(g) => g,
                    Verdict::Refused(r) => {
                        return Err(format!("grow --measure {name} n {n}: {}", r.reason));
                    }
                };
                if let Verdict::Refused(r) = respond(&g, &cells, &frames) {
                    return Err(format!("grow --measure {name} n {n}: {}", r.reason));
                }
                engine = engine.min(start.elapsed());
            }
            steps.push(step.as_micros().to_string());
            engines.push(engine.as_micros().to_string());
        }
        let _ = writeln!(
            out,
            "grow {name} µs per step at n = 0, 6, 24, 96: {} (engine {})",
            steps.join(", "),
            engines.join(", ")
        );
    }
    let _ = writeln!(
        out,
        "grow --measure: {} systems, least of {RUNS} runs (information, not a check)",
        GROW_SYSTEMS.len()
    );
    Ok(out)
}
