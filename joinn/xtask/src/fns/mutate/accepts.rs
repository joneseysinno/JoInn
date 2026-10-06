//! Change what a contact's growing body accepts.

use joinn_dna::{Accept, Contact};
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn accepts(contact: &mut Contact, name: &str, accept: Accept) -> Verdict<()> {
    let Some(grows) = contact.coding.grows.as_mut().filter(|g| g.name == name) else {
        return Verdict::Refused(refuse(format!(
            "no body grows as {name}; acceptance is a contact's growth name"
        )));
    };
    if grows.accepts == accept {
        return Verdict::Refused(refuse(format!(
            "{name} already accepts {}; acceptance is the other word",
            accept.word()
        )));
    }
    grows.accepts = accept;
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
    use joinn_frame::Verdict;

    #[test]
    fn accepts_changes_the_word_and_rebinds_the_system() {
        let mut c = Subject::Contact(growing_contact(Accept::One, None));
        let applied = apply(&mut c, &Mutation::Accepts("numbers", Accept::Any));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let Subject::Contact(contact) = &c else {
            panic!("contact");
        };
        assert_eq!(
            contact.coding.grows.as_ref().map(|g| g.accepts),
            Some(Accept::Any)
        );
        let Verdict::Refused(r) = apply(&mut c, &Mutation::Accepts("numbers", Accept::Any)) else {
            panic!("the same word must refuse");
        };
        assert!(r.reason.contains("already accepts any"), "{}", r.reason);
        let Verdict::Refused(r) = apply(&mut c, &Mutation::Accepts("ghost", Accept::One)) else {
            panic!("a missing growth name must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);

        let mut s = system_fixture();
        let applied = apply(&mut s, &Mutation::Accepts("numbers", Accept::Any));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let Subject::System(sys) = &s else {
            panic!("system");
        };
        let bound = &sys.contacts["numbers"];
        assert_eq!(
            bound.coding.grows.as_ref().map(|g| g.accepts),
            Some(Accept::Any)
        );
        assert_eq!(sys.system.coding.bodies[0].contact, hash(&bound.coding));
        let Verdict::Refused(r) = apply(&mut s, &Mutation::Accepts("ghost", Accept::One)) else {
            panic!("a missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
