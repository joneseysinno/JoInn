//! Re-parse reprinted subject text as the same kind.

use crate::fns::parse_lock_scores::parse_lock_scores;
use crate::fns::subject::{Subject, SystemSubject};
use joinn_dna::{parse_body, parse_contact, parse_system};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::parse_universe;

use super::refuse::refuse;

pub(crate) fn reparse(kind: &Subject, text: &str) -> Verdict<Subject> {
    match kind {
        Subject::Body(_) => match parse_body(text, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => Verdict::Ok(Subject::Body(b)),
            Verdict::Refused(r) => Verdict::Refused(r),
        },
        Subject::Contact(_) => match parse_contact(text, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => Verdict::Ok(Subject::Contact(c)),
            Verdict::Refused(r) => Verdict::Refused(r),
        },
        Subject::System(s) => match parse_system(text) {
            Verdict::Ok(system) => Verdict::Ok(Subject::System(SystemSubject {
                system,
                contacts: s.contacts.clone(),
            })),
            Verdict::Refused(r) => Verdict::Refused(r),
        },
        Subject::Universe(_) => match parse_universe(text) {
            Verdict::Ok(u) => Verdict::Ok(Subject::Universe(u)),
            Verdict::Refused(r) => Verdict::Refused(r),
        },
        Subject::Lock(_) => match parse_lock_scores(text) {
            Ok(rows) => Verdict::Ok(Subject::Lock(rows)),
            Err(reason) => Verdict::Refused(refuse(reason)),
        },
        Subject::Transcript(_) => {
            let lines = text.lines().map(str::to_owned).collect();
            Verdict::Ok(Subject::Transcript(lines))
        }
        Subject::Text(_) => Verdict::Ok(Subject::Text(text.trim().to_owned())),
    }
}
