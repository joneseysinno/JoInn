//! Gate 4 control 3: true when the body's inside is one region.

use super::g4_report::g4_report;
use super::subject::Subject;

pub(crate) fn g4_two_things_control(subject: &Subject) -> bool {
    if !matches!(subject, Subject::Body(_)) {
        return false;
    }
    g4_report(subject).is_some_and(|r| {
        r.regions
            .iter()
            .find(|piece| piece.alias == "body")
            .is_some_and(|piece| piece.pieces.len() == 1)
    })
}
