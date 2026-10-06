//! Drop a lineage from a system: a bound body's (by alias, rebinding the
//! system) or the system's own (by the word `system`).

use joinn_frame::Verdict;

use crate::fns::subject::SystemSubject;

use super::rebind::rebind;
use super::refuse::refuse;

pub(super) fn drop_lineage(s: &mut SystemSubject, name: &str) -> Verdict<()> {
    if let Some(contact) = s.contacts.get_mut(name) {
        if contact.coding.lineage.take().is_none() {
            return Verdict::Refused(refuse(format!(
                "{name} has no lineage; acceptance is a body that names its parent"
            )));
        }
        return rebind(s, name);
    }
    if name != "system" {
        return Verdict::Refused(refuse(format!(
            "no body bound as {name}; acceptance is a system's body alias or the word system"
        )));
    }
    if s.system.coding.lineage.take().is_none() {
        return Verdict::Refused(refuse(
            "the system has no lineage; acceptance is a system that names its parent",
        ));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::Mutation;
    use crate::fns::mutate::apply::apply;
    use crate::fns::mutate::growing_contact::growing_contact;
    use crate::fns::mutate::system_fixture::system_fixture;
    use crate::fns::subject::Subject;
    use joinn_dna::{Accept, hash};
    use joinn_frame::{Hash, Verdict};

    #[test]
    fn drop_lineage_clears_the_body_s_or_the_system_s() {
        let parent = Hash::from_bytes([7; 32]);
        let mut c = Subject::Contact(growing_contact(Accept::Any, Some(parent)));
        let applied = apply(&mut c, &Mutation::DropLineage("numbers"));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let Subject::Contact(contact) = &c else {
            panic!("contact");
        };
        assert_eq!(contact.coding.lineage, None);
        let Verdict::Refused(r) = apply(&mut c, &Mutation::DropLineage("numbers")) else {
            panic!("no lineage left must refuse");
        };
        assert!(r.reason.contains("no lineage"), "{}", r.reason);

        let mut s = system_fixture();
        let Subject::System(sys) = &mut s else {
            panic!("system");
        };
        sys.system.coding.lineage = Some(parent);
        if let Some(bound) = sys.contacts.get_mut("numbers") {
            bound.coding.lineage = Some(parent);
        }
        let applied = apply(&mut s, &Mutation::DropLineage("numbers"));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let applied = apply(&mut s, &Mutation::DropLineage("system"));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let Subject::System(sys) = &s else {
            panic!("system");
        };
        let bound = &sys.contacts["numbers"];
        assert_eq!(bound.coding.lineage, None);
        assert_eq!(sys.system.coding.bodies[0].contact, hash(&bound.coding));
        assert_eq!(sys.system.coding.lineage, None);
        let Verdict::Refused(r) = apply(&mut s, &Mutation::DropLineage("ghost")) else {
            panic!("a missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
