//! Whether a gate subject is refused by its own kind's admission.

use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{BodyStore, check_contact};

use super::corpus_store::corpus_store;
use super::forces::corpus_cells;
use super::grove::admit_universe;
use super::subject::Subject;
use super::system_admission::system_admission;

/// A universe by `admit_universe` against the corpus store, a body by a store
/// insert against the corpus cells, a contact by `check_contact`, a system by
/// `check_system` against the corpus. A lock, a transcript or a text has no
/// admission and is never refused by one.
pub(crate) fn refused_at_admission(subject: &Subject) -> bool {
    match subject {
        Subject::Universe(u) => corpus_store()
            .is_ok_and(|(_, store)| matches!(admit_universe(u, store), Verdict::Refused(_))),
        Subject::Body(b) => corpus_store().is_ok_and(|(cells, _)| {
            matches!(
                BodyStore::new().insert(b.clone(), cells.clone(), "gate subject"),
                Verdict::Refused(_)
            )
        }),
        Subject::Contact(c) => {
            let frames = FrameRegistry::phase1();
            corpus_cells(&frames)
                .is_ok_and(|cells| matches!(check_contact(c, &cells, &frames), Verdict::Refused(_)))
        }
        Subject::System(s) => {
            matches!(system_admission(s), Ok(Verdict::Refused(_)))
        }
        Subject::Lock(_) | Subject::Transcript(_) | Subject::Text(_) => false,
    }
}
