//! Apply a catalogue mutation, reprint, re-parse, and require a hash move.

use crate::fns::subject::Subject;
use joinn_frame::Verdict;

use super::Mutation;
use super::apply::apply;
use super::coding_hash::coding_hash;
use super::refuse::refuse;
use super::reparse::reparse;
use super::reprint::reprint;

/// Apply `m` to `s`, reprint with the kind's printer, and re-parse.
pub(crate) fn mutate(s: &Subject, m: &Mutation) -> Verdict<Subject> {
    let before = coding_hash(s);
    let mut next = s.clone();
    match apply(&mut next, m) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let text = reprint(&next);
    let parsed = match reparse(s, &text) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if let (Some(a), Some(b)) = (before, coding_hash(&parsed)) {
        if a == b {
            return Verdict::Refused(refuse(
                "mutation did not change the coding hash; acceptance is a coding change",
            ));
        }
    }
    Verdict::Ok(parsed)
}
