//! Drop a growing contact's lineage, named by its growth name.

use joinn_dna::Contact;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn drop_contact_lineage(contact: &mut Contact, name: &str) -> Verdict<()> {
    if contact.coding.grows.as_ref().is_none_or(|g| g.name != name) {
        return Verdict::Refused(refuse(format!(
            "no body grows as {name}; acceptance is a contact's growth name"
        )));
    }
    if contact.coding.lineage.take().is_none() {
        return Verdict::Refused(refuse(format!(
            "{name} has no lineage; acceptance is a contact that names its parent"
        )));
    }
    Verdict::Ok(())
}
