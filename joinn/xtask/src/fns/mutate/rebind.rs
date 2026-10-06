//! Point a system's body at its bound contact's current hash.

use joinn_dna::hash;
use joinn_frame::Verdict;

use crate::fns::subject::SystemSubject;

use super::refuse::refuse;

pub(super) fn rebind(s: &mut SystemSubject, alias: &str) -> Verdict<()> {
    let (Some(contact), Some(body)) = (
        s.contacts.get(alias),
        s.system.coding.bodies.iter_mut().find(|b| b.alias == alias),
    ) else {
        return Verdict::Refused(refuse(format!(
            "no body bound as {alias}; acceptance is a system's body alias"
        )));
    };
    body.contact = hash(&contact.coding);
    Verdict::Ok(())
}
