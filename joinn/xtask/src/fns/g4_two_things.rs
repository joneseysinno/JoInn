//! Gate 4 item 3: a body that is two things is named.

use joinn_link::print_assay;

use super::g4_report::g4_report;
use super::g4_subject::g4_subject;

/// `phase52/adversary/asker.body` prints `regions: body 2 {answer} {question}`.
pub(crate) fn g4_two_things() -> bool {
    let Some(subject) = g4_subject("phase52/adversary/asker.body") else {
        return false;
    };
    let Some(report) = g4_report(&subject) else {
        return false;
    };
    print_assay(&report).contains("regions: body 2 {answer} {question}\n")
}
