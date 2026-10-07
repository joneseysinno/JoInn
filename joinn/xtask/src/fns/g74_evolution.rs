//! Gate 7.4 item 2: evolution keeps every old witness, and gains.

use joinn_dna::{Accept, hash};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::check_evolution;

use super::forces::corpus_cells;
use super::grow::corpus_system;
use super::mutate::{Mutation, mutate};
use super::subject::Subject;
use super::system_subject::system_subject;

const NOTHING_NEW: &str = "evolution: counting accepts nothing counting refuses; acceptance is a new ability (otherwise it is an edit)";

/// `check_evolution(counting, adding)` holds with 3 witnesses; adding made to
/// accept one (a counting that names counting as its parent and gains
/// nothing) is refused with §2.9's words.
pub(crate) fn g74_evolution() -> bool {
    let frames = FrameRegistry::phase1();
    let loaded = (
        corpus_cells(&frames),
        corpus_system("counting"),
        corpus_system("adding"),
    );
    let (cells, parent, child) = match loaded {
        (Ok(c), Ok(p), Ok(ch)) => (c, p, ch),
        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
            println!("{e}");
            return false;
        }
    };
    let mut contacts = parent.contacts.clone();
    contacts.extend(child.contacts.clone());
    let held = match check_evolution(&parent.system, &child.system, &contacts, &cells) {
        Verdict::Ok(e) => {
            println!(
                "evolution counting → adding: {} witnesses hold; adding gains {}",
                e.witnesses,
                e.gained.print_term()
            );
            e.witnesses == 3
        }
        Verdict::Refused(r) => {
            println!("evolution counting → adding: refused: {}", r.reason);
            false
        }
    };
    let edit = match system_subject(child.system.clone()).map(Subject::System) {
        Ok(s) => mutate(&s, &Mutation::Accepts("numbers", Accept::One)),
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let Verdict::Ok(Subject::System(edit)) = edit else {
        println!("evolution: adding accepting one could not be made");
        return false;
    };
    let mut contacts = parent.contacts.clone();
    contacts.extend(edit.contacts.values().map(|c| (hash(&c.coding), c.clone())));
    let refused = match check_evolution(&parent.system, &edit.system, &contacts, &cells) {
        Verdict::Refused(r) => {
            println!(
                "evolution counting → adding accepting one: refused: {}",
                r.reason
            );
            r.reason == NOTHING_NEW
        }
        Verdict::Ok(e) => {
            println!(
                "evolution counting → adding accepting one: {} witnesses hold, gained {}; acceptance is a refusal",
                e.witnesses,
                e.gained.print_term()
            );
            false
        }
    };
    held && refused
}
