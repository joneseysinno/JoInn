//! Honest fixture control: catches DropLink(e0), ignores a neutral edit.

use crate::fns::subject::Subject;

pub(super) fn honest_crossing(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    u.coding.links.iter().all(|l| l.id != "e0")
}
