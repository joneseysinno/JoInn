//! Plan 7.4's systems: tables grown by deltas equal tables regrown from the
//! system and the whole input list, at every size (rule 58).

use joinn_dna::Accept;
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Verdict};
use joinn_visual::{SystemScene, table_bytes};

use crate::fns::forces::corpus_cells;
use crate::fns::grow::{GROW_SYSTEMS, corpus_system};

/// What each body accepts, seven times: counting `1`, adding seven others.
const COUNTING: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
const ADDING: [i64; 7] = [2, 3, 4, -2, 0, 7, 12];

/// Per system `system <name>: deltas equal regrow at n = 0 … 7`. A size whose
/// tables differ from a regrow, or a step the body refuses, fails.
pub(crate) fn system_pass(
    lines: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Result<(), String> {
    let cells = corpus_cells(&FrameRegistry::phase1())?;
    for name in GROW_SYSTEMS {
        let s = corpus_system(name)?;
        let inputs = match s.accepts {
            Accept::One => COUNTING,
            Accept::Any => ADDING,
        };
        let mut scene = match SystemScene::grow(&s.system, &s.contacts, &cells, s.waiting) {
            Verdict::Ok(sc) => sc,
            Verdict::Refused(r) => return Err(format!("regrow: system {name}: {}", r.reason)),
        };
        let mut so_far = Vec::new();
        let mut differ = Vec::new();
        for n in 0..=inputs.len() {
            if let Some(v) = n.checked_sub(1).and_then(|i| inputs.get(i)) {
                let value = match IntFrame::new().canonicalize(Term::int(*v)) {
                    Verdict::Ok(v) => v,
                    Verdict::Refused(r) => return Err(r.reason),
                };
                if let Verdict::Refused(r) = scene.apply_growth(&value) {
                    return Err(format!("regrow: system {name} n {n}: {}", r.reason));
                }
                so_far.push(value);
            }
            let fresh =
                match SystemScene::regrow(&s.system, &s.contacts, &cells, s.waiting, &so_far) {
                    Verdict::Ok(f) => f,
                    Verdict::Refused(r) => {
                        return Err(format!("regrow: system {name} n {n}: {}", r.reason));
                    }
                };
            if table_bytes(scene.tables()) != table_bytes(fresh.tables()) {
                differ.push(n.to_string());
                failures.push(format!(
                    "system {name} n {n}: the tables grown by deltas differ from regrow (rule 58)"
                ));
            }
        }
        if differ.is_empty() {
            lines.push(format!(
                "system {name}: deltas equal regrow at n = 0 … {}",
                inputs.len()
            ));
        } else {
            lines.push(format!(
                "system {name}: deltas differ from regrow at n = {}",
                differ.join(", ")
            ));
        }
    }
    Ok(())
}
