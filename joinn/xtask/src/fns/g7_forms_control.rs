//! Gate 7 item 1 control: the contact is refused, runs differently in the CLI,
//! or sums to something other than 5.

use std::fs;

use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::lower;

use super::forces::corpus_cells;
use super::g7_sum_after::g7_sum_after;
use super::g7_transcript::g7_transcript;
use super::subject::Subject;
use super::workspace_root;

/// True when admission refuses the subject, its CLI transcript differs from
/// `transcripts/calculator.txt`, or `sum@2` after the script is not `5`.
/// Descriptions are the check's: the neutral edit changes `sum`'s label.
pub(crate) fn g7_forms_control(subject: &Subject) -> bool {
    let Subject::Contact(contact) = subject else {
        return false;
    };
    let frames = FrameRegistry::phase1();
    let Ok(cells) = corpus_cells(&frames) else {
        return false;
    };
    if let Verdict::Refused(_) = lower(contact, &cells, &frames) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return false;
    };
    let golden = root
        .join("corpus")
        .join("transcripts")
        .join("calculator.txt");
    let Ok(want) = fs::read_to_string(golden) else {
        return false;
    };
    match g7_transcript(contact) {
        Ok(got) if got == want.replace("\r\n", "\n") => {}
        _ => return true,
    }
    g7_sum_after(contact, &cells).as_deref() != Some("5")
}
