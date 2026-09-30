//! Drop a genome instance from a contact (and its entry if emptied).

use joinn_dna::Contact;
use joinn_frame::Verdict;

use super::refuse::refuse;

/// Grants that named the instance lose it. Forces keep their members: a
/// member that names no instance is what admission must refuse.
pub(super) fn drop_contact_genome(contact: &mut Contact, instance: &str) -> Verdict<()> {
    let mut found = false;
    for entry in &mut contact.coding.genome {
        let before = entry.instances.len();
        entry.instances.retain(|i| i != instance);
        found |= entry.instances.len() != before;
    }
    if !found {
        return Verdict::Refused(refuse(format!(
            "no such instance {instance}; acceptance is a genome instance"
        )));
    }
    contact.coding.genome.retain(|e| !e.instances.is_empty());
    for insts in contact.coding.grants.values_mut() {
        insts.retain(|i| i != instance);
    }
    contact.coding.grants.retain(|_, insts| !insts.is_empty());
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::contact_admission::contact_admission;
    use crate::fns::mutate::contact_fixture::contact_fixture;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::subject::Subject;
    use joinn_dna::hash;
    use joinn_frame::Verdict;

    #[test]
    fn drop_genome_keeps_the_member_that_names_it() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropGenome("cli_b")) else {
            panic!("mutate");
        };
        let Subject::Contact(c) = &mutant else {
            panic!("contact mutant");
        };
        assert_ne!(hash(&c.coding), hash(&orig.coding));
        assert!(
            c.coding
                .genome
                .iter()
                .all(|g| g.instances.iter().all(|i| i != "cli_b"))
        );
        assert_eq!(
            c.coding.grants.get("stdin").map(Vec::as_slice),
            Some(&["cli_a".to_string()][..])
        );
        assert!(
            c.coding.forces[0]
                .members
                .iter()
                .any(|m| m.instance == "cli_b")
        );
        let Verdict::Refused(r) = contact_admission(&mutant) else {
            panic!("a member of a dropped instance must be refused");
        };
        assert_eq!(
            r.reason,
            "member cli_b@1 names no instance; acceptance is an instance of the genome or a force's response"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropGenome("ghost")) else {
            panic!("missing instance must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
