//! Gate 7 item 2 control: a member moved to a port that won't receive it.

use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::check_contact;

use super::forces::corpus_cells;
use super::subject::Subject;

/// True when `check_contact` refuses the subject at a receptor, naming `cli_a@0`.
pub(crate) fn g7_order_blind_control(subject: &Subject) -> bool {
    let Subject::Contact(contact) = subject else {
        return false;
    };
    let frames = FrameRegistry::phase1();
    let Ok(cells) = corpus_cells(&frames) else {
        return false;
    };
    match check_contact(contact, &cells, &frames) {
        Verdict::Refused(r) => r.reason.starts_with("receptor") && r.reason.contains("cli_a@0"),
        Verdict::Ok(_) => false,
    }
}
