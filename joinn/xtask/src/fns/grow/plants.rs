//! `grow`'s standing plants (rule 93): truths no corpus system can break.

use joinn_dna::{Cell, hash, print_system};
use joinn_frame::{
    CanonWriter, Frame, FrameRegistry, Hash, IntFrame, TAG_SYSTEM, Term, Verdict, keyed_hash,
};
use joinn_link::{count_witness, grow as grown_from, respond};
use joinn_visual::{LASSO_GAP, check_lasso, lasso_outline, layout_system};
use std::collections::BTreeMap;

use super::{CorpusSystem, step_fault};

/// On `s` grown by `1 1 1`, each plant must be refused by `step_fault`:
/// (a) the fold drops its last member (its response is the fold of all but
/// the last input), refused at the first n ≥ 2; (b) the lasso's outline drawn
/// 2 units inward, refused by lasso rule 1; (c) the hash is computed over the
/// grown state (the canonical print, then each input), refused at the first
/// growth. Returns the plants' lines and the names of those not refused.
pub(crate) fn plants(
    s: &CorpusSystem,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<(Vec<String>, Vec<&'static str>), String> {
    let frames = FrameRegistry::phase1();
    let one = match IntFrame::new().canonicalize(Term::int(1)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let inputs = vec![one; 3];
    let at = |n: usize| match grown_from(&s.system, &s.contacts, cells, &inputs[..n]) {
        Verdict::Ok(g) => Ok(g),
        Verdict::Refused(r) => Err(format!("plant: {}", r.reason)),
    };
    let answer = |n: usize| match respond(&at(n)?, cells, &frames) {
        Verdict::Ok(v) => Ok(v),
        Verdict::Refused(r) => Err(format!("plant: {}", r.reason)),
    };
    let witness = |n: usize| match count_witness(&inputs[..n]) {
        Verdict::Ok(v) => Ok(v),
        Verdict::Refused(r) => Err(format!("plant: {}", r.reason)),
    };
    let true_hash = hash(&s.system.coding);
    let mut lines = Vec::new();
    let mut missed = Vec::new();
    let mut fold = None;
    let mut grown_hash = None;
    for n in 0..=inputs.len() {
        let dropped = answer(if n >= 2 { n - 1 } else { n })?;
        if fold.is_none() {
            fold = step_fault(
                &s.name,
                &inputs[..n],
                &dropped,
                &witness(n)?,
                &true_hash,
                &s.golden,
            )
            .map(|f| (n, f));
        }
        let mut w = CanonWriter::new();
        w.push_str(&print_system(&s.system.coding));
        for v in &inputs[..n] {
            w.push_str(&v.print_literal());
        }
        let over_state = keyed_hash(TAG_SYSTEM, &w.finish());
        if grown_hash.is_none() {
            grown_hash = step_fault(
                &s.name,
                &inputs[..n],
                &answer(n)?,
                &witness(n)?,
                &over_state,
                &s.golden,
            )
            .map(|f| (n, f));
        }
    }
    match fold {
        Some((n, f)) => lines.push(format!(
            "planted fold drops its last member: refused at n {n}: {f}"
        )),
        None => missed.push("fold"),
    }
    let n = inputs.len();
    let mut laid = match layout_system(&s.system, &s.contacts, cells, &at(n)?, s.waiting) {
        Verdict::Ok(l) => l,
        Verdict::Refused(r) => return Err(format!("plant: {}", r.reason)),
    };
    let inward = lasso_outline(&laid.surface, LASSO_GAP - 2);
    laid.lasso.splice(..inward.len(), inward);
    match check_lasso(&laid) {
        Verdict::Refused(r) => lines.push(format!(
            "planted lasso drawn 2 units inward: refused at n {n}: {}",
            r.reason
        )),
        Verdict::Ok(()) => missed.push("lasso"),
    }
    match grown_hash {
        Some((n, f)) => lines.push(format!(
            "planted hash over the grown state: refused at n {n}: {f}"
        )),
        None => missed.push("hash"),
    }
    Ok((lines, missed))
}
