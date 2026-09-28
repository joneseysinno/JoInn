//! Gate 4 control 1: true when the body's assay prints H₁ ≠ 0.

use super::g4_report::g4_report;
use super::subject::Subject;

pub(crate) fn g4_sample_control(subject: &Subject) -> bool {
    if !matches!(subject, Subject::Body(_)) {
        return false;
    }
    g4_report(subject).is_some_and(|r| r.b1 != 0)
}
