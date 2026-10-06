//! Reprint and re-parse a system and each contact it binds.

use crate::fns::subject::{Subject, SystemSubject};
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::reparse::reparse;
use super::reprint::reprint;

pub(super) fn reparse_system(next: &SystemSubject) -> Verdict<Subject> {
    let mut contacts = BTreeMap::new();
    for (alias, contact) in &next.contacts {
        let kind = Subject::Contact(contact.clone());
        match reparse(&kind, &reprint(&kind)) {
            Verdict::Ok(Subject::Contact(c)) => {
                contacts.insert(alias.clone(), c);
            }
            Verdict::Ok(_) => {
                return Verdict::Refused(refuse(format!(
                    "{alias} re-parsed as another kind; acceptance is a contact"
                )));
            }
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    let kind = Subject::System(SystemSubject {
        system: next.system.clone(),
        contacts,
    });
    reparse(&kind, &reprint(&kind))
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::system_fixture::system_fixture;
    use crate::fns::mutate::{Mutation, mutate, neutral};
    use crate::fns::subject::Subject;
    use joinn_dna::{Accept, hash};
    use joinn_frame::Verdict;

    #[test]
    fn every_system_mutation_survives_the_round_trip_and_moves_the_hash() {
        let s = system_fixture();
        let Subject::System(orig) = &s else {
            panic!("system");
        };
        let before = hash(&orig.system.coding);
        for m in [
            Mutation::Accepts("numbers", Accept::Any),
            Mutation::DropForce("count"),
            Mutation::ForceOn("count", "ghost"),
        ] {
            let Verdict::Ok(Subject::System(next)) = mutate(&s, &m) else {
                panic!("{m:?}");
            };
            assert_ne!(hash(&next.system.coding), before, "{m:?}");
            if m == Mutation::Accepts("numbers", Accept::Any) {
                let bound = &next.contacts["numbers"];
                assert_eq!(
                    bound.coding.grows.as_ref().map(|g| g.accepts),
                    Some(Accept::Any)
                );
                assert_eq!(next.system.coding.bodies[0].contact, hash(&bound.coding));
            }
        }
        let Some(Subject::System(n)) = neutral(&s) else {
            panic!("neutral");
        };
        assert_eq!(hash(&n.system.coding), before);
        assert_ne!(n.system.regulatory, orig.system.regulatory);
    }
}
