//! Drop the force whose response is named.

use joinn_dna::Contact;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn drop_force(contact: &mut Contact, response: &str) -> Verdict<()> {
    let before = contact.coding.forces.len();
    contact.coding.forces.retain(|f| f.name != response);
    if contact.coding.forces.len() == before {
        return Verdict::Refused(refuse(format!(
            "no force responds as {response}; acceptance is a force's response name"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::contact_fixture::contact_fixture;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::subject::Subject;
    use joinn_dna::{hash, print_contact};
    use joinn_frame::Verdict;

    #[test]
    fn drop_force_leaves_no_sum() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropForce("sum")) else {
            panic!("mutate");
        };
        let Subject::Contact(c) = &mutant else {
            panic!("contact mutant");
        };
        assert_ne!(hash(&c.coding), hash(&orig.coding));
        assert!(c.coding.forces.is_empty());
        let text = print_contact(&c.coding);
        assert!(!text.contains("sum"), "{text}");
        assert!(text.ends_with("lineage none\nforces\n"), "{text}");
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropForce("ghost")) else {
            panic!("missing force must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
