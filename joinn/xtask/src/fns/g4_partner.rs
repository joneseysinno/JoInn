//! Gate 4 item 2: a promise names its partner.

use super::g4_report::g4_report;
use super::g4_subject::g4_subject;

/// `phase4/loop.universe` prints `H₁: 0`, the ring filled by `calc.cli_a roundtrip`.
pub(crate) fn g4_partner() -> bool {
    let Some(subject) = g4_subject("phase4/loop.universe") else {
        return false;
    };
    let Some(report) = g4_report(&subject) else {
        return false;
    };
    report.b1 == 0
        && report.open.is_empty()
        && report.filled.len() == 1
        && report
            .filled
            .iter()
            .all(|l| l.ends_with(" by calc.cli_a roundtrip"))
}
