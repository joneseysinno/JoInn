//! Test fixture: admit a contact subject against the corpus cells.

use crate::fns::forces::corpus_cells;
use crate::fns::subject::Subject;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::check_contact;

pub(crate) fn contact_admission(s: &Subject) -> Verdict<()> {
    let Subject::Contact(contact) = s else {
        panic!("contact subject");
    };
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames).unwrap_or_else(|e| panic!("{e}"));
    check_contact(contact, &cells, &frames)
}
