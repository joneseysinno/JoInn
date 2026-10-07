//! Gate 7.4 item 1 control: the subject is refused, accepts what counting
//! refuses, or a size is untrue.

use joinn_dna::hash;
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Verdict};
use joinn_link::{count_witness, grow, grow_step, respond};
use std::collections::BTreeMap;

use super::forces::corpus_cells;
use super::refused_at_admission::refused_at_admission;
use super::subject::Subject;

/// True when the system is refused at admission, or growing it by `1 1 3`
/// accepts the `3` (or refuses a `1`), or a response at any size differs from
/// `count_witness`.
pub(crate) fn g74_growth_control(subject: &Subject) -> bool {
    let Subject::System(s) = subject else {
        return false;
    };
    if refused_at_admission(subject) {
        return true;
    }
    let frames = FrameRegistry::phase1();
    let Ok(cells) = corpus_cells(&frames) else {
        return true;
    };
    let contacts: BTreeMap<_, _> = s
        .contacts
        .values()
        .map(|c| (hash(&c.coding), c.clone()))
        .collect();
    let Verdict::Ok(mut grown) = grow(&s.system, &contacts, &cells, &[]) else {
        return true;
    };
    let untrue =
        |g: &joinn_link::Grown| match (respond(g, &cells, &frames), count_witness(g.inputs())) {
            (Verdict::Ok(r), Verdict::Ok(w)) => r != w,
            _ => true,
        };
    if untrue(&grown) {
        return true;
    }
    for v in [1, 1, 3] {
        let Verdict::Ok(value) = IntFrame::new().canonicalize(Term::int(v)) else {
            return true;
        };
        match grow_step(&grown, &value) {
            Verdict::Ok(g) => grown = g,
            Verdict::Refused(_) => return v != 3,
        }
        if v == 3 || untrue(&grown) {
            return true;
        }
    }
    false
}
