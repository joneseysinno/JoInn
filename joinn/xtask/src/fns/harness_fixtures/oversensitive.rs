//! Fixture control that flips on any change, including the neutral edit.

use crate::fns::subject::Subject;

pub(super) fn oversensitive(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return true;
    };
    let missing_e0 = u.coding.links.iter().all(|l| l.id != "e0");
    let neutral_touched = u
        .regulatory
        .labels
        .values()
        .any(|l| l.contains("(neutral)"))
        || u.regulatory
            .names
            .values()
            .any(|n| n.contains("(neutral)"));
    missing_e0 || neutral_touched
}
