//! `cargo xtask grow [--measure]`.

use joinn_dna::hash;
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Value, Verdict};
use joinn_link::{
    Grown, accept_word, check_evolution, count_witness, grow as grown_from, grow_step, respond,
    transcripts,
};
use joinn_visual::{check_lasso, layout_system};

use super::plants::plants;
use super::{CorpusSystem, GROW_SYSTEMS, corpus_system, grow_measure, step_fault};
use crate::fns::forces::corpus_cells;

/// Per system and §3 transcript, one line per step:
/// `grow <system> <step>: +<v> → cells <n>, count <c>, witness <c>, lasso ok,
/// hash <12 hex>` (an empty transcript prints step 0), or `… +<v> refused:
/// <reason>`, which ends that transcript. Then the evolution line, the
/// standing plants (rule 93) and the last line. A V161 or V162 fault, a lasso
/// rule broken, a refused evolution or a plant not refused fails. With
/// `--measure`, the step costs instead.
pub(crate) fn grow(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [] => {}
        [flag] if flag == "--measure" => {
            print!("{}", grow_measure()?);
            return Ok(());
        }
        _ => return Err("usage: cargo xtask grow [--measure]".into()),
    }
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames)?;
    let int = |v: i64| match IntFrame::new().canonicalize(Term::int(v)) {
        Verdict::Ok(v) => Ok(v),
        Verdict::Refused(r) => Err(r.reason),
    };
    let systems: Vec<CorpusSystem> = GROW_SYSTEMS
        .iter()
        .map(|n| corpus_system(n))
        .collect::<Result<_, _>>()?;
    let mut faults = Vec::new();
    for s in &systems {
        let name = s.name.as_str();
        let report = |g: &Grown, faults: &mut Vec<String>| -> Result<String, String> {
            let response = match respond(g, &cells, &frames) {
                Verdict::Ok(v) => v,
                Verdict::Refused(r) => return Err(format!("grow {name}: {}", r.reason)),
            };
            let witness = match count_witness(g.inputs()) {
                Verdict::Ok(v) => v,
                Verdict::Refused(r) => return Err(format!("grow {name}: {}", r.reason)),
            };
            let h = hash(&g.system().coding);
            if let Some(f) = step_fault(name, g.inputs(), &response, &witness, &h, &s.golden) {
                faults.push(f);
            }
            let lasso = match layout_system(&s.system, &s.contacts, &cells, g, s.waiting) {
                Verdict::Ok(l) => check_lasso(&l),
                Verdict::Refused(r) => Verdict::Refused(r),
            };
            let lasso = match lasso {
                Verdict::Ok(()) => "ok",
                Verdict::Refused(r) => {
                    faults.push(format!("grow {name} {}: {}", g.inputs().len(), r.reason));
                    "refused"
                }
            };
            Ok(format!(
                "cells {}, count {}, witness {}, lasso {lasso}, hash {}",
                g.inputs().len(),
                response.print_term(),
                witness.print_term(),
                &h.to_hex()[..12]
            ))
        };
        for transcript in transcripts(s.accepts) {
            let mut grown = match grown_from(&s.system, &s.contacts, &cells, &[]) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => return Err(format!("grow {name}: {}", r.reason)),
            };
            if transcript.is_empty() {
                println!("grow {name} 0: (none) → {}", report(&grown, &mut faults)?);
            }
            for (step, v) in (1..).zip(&transcript) {
                let value: Value = int(*v)?;
                match grow_step(&grown, &value) {
                    Verdict::Ok(g) => {
                        grown = g;
                        println!(
                            "grow {name} {step}: +{v} → {}",
                            report(&grown, &mut faults)?
                        );
                    }
                    Verdict::Refused(r) => {
                        println!("grow {name} {step}: +{v} refused: {}", r.reason);
                        break;
                    }
                }
            }
        }
    }
    for f in &faults {
        println!("{f}");
    }
    let [parent, child] = systems.as_slice() else {
        return Err("grow: two systems expected".into());
    };
    let mut contacts = parent.contacts.clone();
    contacts.extend(child.contacts.clone());
    let (pw, cw) = (accept_word(parent.accepts), accept_word(child.accepts));
    let evolved = match check_evolution(&parent.system, &child.system, &contacts, &cells) {
        Verdict::Ok(e) => {
            println!(
                "evolve {pw} → {cw}: {} witnesses hold; {cw} accepts {}, {pw} refuses it",
                e.witnesses,
                e.gained.print_term()
            );
            true
        }
        Verdict::Refused(r) => {
            println!("evolve {pw} → {cw}: refused: {}", r.reason);
            false
        }
    };
    let (planted, missed) = plants(parent, &cells)?;
    for line in &planted {
        println!("{line}");
    }
    if !faults.is_empty() {
        return Err(format!("grow: {} fault(s)", faults.len()));
    }
    if !evolved {
        return Err("grow: evolution refused".into());
    }
    if !missed.is_empty() {
        return Err(format!(
            "grow: planted {} not refused; acceptance is refused (rule 93)",
            missed.join(", ")
        ));
    }
    println!(
        "grow: {} systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok)",
        systems.len()
    );
    Ok(())
}
