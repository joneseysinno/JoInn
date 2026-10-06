//! Change what a system's bound body accepts, and rebind the system to it.

use joinn_dna::Accept;
use joinn_frame::Verdict;

use crate::fns::subject::SystemSubject;

use super::accepts::accepts;
use super::rebind::rebind;
use super::refuse::refuse;

pub(super) fn system_accepts(s: &mut SystemSubject, alias: &str, accept: Accept) -> Verdict<()> {
    let Some(contact) = s.contacts.get_mut(alias) else {
        return Verdict::Refused(refuse(format!(
            "no body bound as {alias}; acceptance is a system's body alias"
        )));
    };
    let name = contact.coding.grows.as_ref().map(|g| g.name.clone());
    let Some(name) = name else {
        return Verdict::Refused(refuse(format!(
            "{alias} does not grow; acceptance is a body with a grows section"
        )));
    };
    match accepts(contact, &name, accept) {
        Verdict::Ok(()) => rebind(s, alias),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}
