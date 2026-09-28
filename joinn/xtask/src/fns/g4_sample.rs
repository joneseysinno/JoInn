//! Gate 4 item 1: the instrument reads a known sample.

use super::g4_report::g4_report;
use super::g4_subject::g4_subject;

/// `phase2/calculator.body` prints `H₁: 0`, both loops filled by `roundtrip`.
pub(crate) fn g4_sample() -> bool {
    let Some(subject) = g4_subject("phase2/calculator.body") else {
        return false;
    };
    let Some(report) = g4_report(&subject) else {
        return false;
    };
    report.b1 == 0
        && report.open.is_empty()
        && report.filled.len() == 2
        && report
            .filled
            .iter()
            .any(|l| l.ends_with(" by body.cli_a roundtrip"))
        && report
            .filled
            .iter()
            .any(|l| l.ends_with(" by body.cli_b roundtrip"))
}
