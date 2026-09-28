//! Gate 4 item 4: a declaration refuses; an assay doesn't.

use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind};

use super::corpus_store::corpus_store;
use super::g4_subject::g4_subject;
use super::gate_four_items::FMT_TWIN_HASH;
use super::mutate::{Mutation, mutate};
use super::subject::Subject;

/// `phase4/loop_declared.universe` is admitted, and `phase4/loop.universe`
/// with the `fmt_twin` binding still assembles: its open loop is only reported.
pub(crate) fn g4_declared() -> bool {
    let Ok((_, store)) = corpus_store() else {
        return false;
    };
    let admitted = |subject: &Subject| {
        let Subject::Universe(universe) = subject else {
            return false;
        };
        let Verdict::Ok(bound) = bind(universe, store) else {
            return false;
        };
        matches!(assemble_universe(universe, &bound), Verdict::Ok(()))
    };
    let Some(declared) = g4_subject("phase4/loop_declared.universe") else {
        return false;
    };
    let Some(plain) = g4_subject("phase4/loop.universe") else {
        return false;
    };
    let Verdict::Ok(plain_twin) = mutate(&plain, &Mutation::SwapBinding("fmt", FMT_TWIN_HASH))
    else {
        return false;
    };
    admitted(&declared) && admitted(&plain_twin)
}
