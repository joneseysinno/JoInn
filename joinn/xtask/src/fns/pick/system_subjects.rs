//! Plan 7.4's systems as scenes at sizes 0, 3 and 7, with their CPU picks.

use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Verdict};
use joinn_visual::{SystemScene, cpu_pick};

use super::SystemSubject;
use crate::fns::forces::corpus_cells;
use crate::fns::grow::{GROW_SYSTEMS, corpus_system, seven_inputs};

/// The sizes `pick` draws each system at.
const SIZES: [usize; 3] = [0, 3, 7];

/// Each corpus system regrown from the first `n` of its seven inputs, fitted
/// to 1280×720. The label is `system <name> n <n>`.
pub(crate) fn system_subjects() -> Result<Vec<SystemSubject>, String> {
    let cells = corpus_cells(&FrameRegistry::phase1())?;
    let mut out = Vec::new();
    for name in GROW_SYSTEMS {
        let s = corpus_system(name)?;
        let mut inputs = Vec::new();
        for v in seven_inputs(s.accepts) {
            match IntFrame::new().canonicalize(Term::int(v)) {
                Verdict::Ok(v) => inputs.push(v),
                Verdict::Refused(r) => return Err(r.reason),
            }
        }
        for n in SIZES {
            let grown = inputs.get(..n).unwrap_or(&inputs);
            let scene = match SystemScene::regrow(&s.system, &s.contacts, &cells, s.waiting, grown)
            {
                Verdict::Ok(sc) => sc,
                Verdict::Refused(r) => return Err(format!("pick: system {name}: {}", r.reason)),
            };
            let camera = scene.fit(1280, 720);
            let cpu = match cpu_pick(&scene.shapes(), &camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => return Err(format!("pick: system {name}: {}", r.reason)),
            };
            out.push((format!("system {name} n {n}"), scene, camera, cpu));
        }
    }
    Ok(out)
}
