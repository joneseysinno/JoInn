//! Gate 7.4 item 2 control: the subject is refused, or it does not evolve
//! counting.

use joinn_dna::hash;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::check_evolution;

use super::forces::corpus_cells;
use super::grow::corpus_system;
use super::refused_at_admission::refused_at_admission;
use super::subject::Subject;

/// True when the system is refused at admission, or
/// `check_evolution(counting, subject)` refuses it.
pub(crate) fn g74_evolution_control(subject: &Subject) -> bool {
    let Subject::System(s) = subject else {
        return false;
    };
    if refused_at_admission(subject) {
        return true;
    }
    let (Ok(cells), Ok(parent)) = (
        corpus_cells(&FrameRegistry::phase1()),
        corpus_system("counting"),
    ) else {
        return true;
    };
    let mut contacts = parent.contacts.clone();
    contacts.extend(s.contacts.values().map(|c| (hash(&c.coding), c.clone())));
    matches!(
        check_evolution(&parent.system, &s.system, &contacts, &cells),
        Verdict::Refused(_)
    )
}
