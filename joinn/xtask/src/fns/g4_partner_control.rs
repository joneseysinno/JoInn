//! Gate 4 control 2: true when the universe's assay has an `open:` line.

use super::g4_report::g4_report;
use super::subject::Subject;

pub(crate) fn g4_partner_control(subject: &Subject) -> bool {
    if !matches!(subject, Subject::Universe(_)) {
        return false;
    }
    g4_report(subject).is_some_and(|r| !r.open.is_empty())
}
