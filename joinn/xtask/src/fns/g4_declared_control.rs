//! Gate 4 control 4: true when assembly refuses naming `assert H₁ = 0`.

use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind};

use super::corpus_store::corpus_store;
use super::subject::Subject;

pub(crate) fn g4_declared_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return false;
    };
    let Ok((_, store)) = corpus_store() else {
        return false;
    };
    let Verdict::Ok(bound) = bind(universe, store) else {
        return false;
    };
    match assemble_universe(universe, &bound) {
        Verdict::Refused(r) => r.reason.contains("assert H₁ = 0"),
        Verdict::Ok(()) => false,
    }
}
