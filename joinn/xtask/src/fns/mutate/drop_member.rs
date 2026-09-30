//! Drop one member of a force.

use joinn_dna::Contact;
use joinn_frame::Verdict;

use super::parse_contact_member::parse_contact_member;
use super::refuse::refuse;

pub(super) fn drop_member(contact: &mut Contact, response: &str, member_s: &str) -> Verdict<()> {
    let want = match parse_contact_member(member_s) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let Some(force) = contact
        .coding
        .forces
        .iter_mut()
        .find(|f| f.name == response)
    else {
        return Verdict::Refused(refuse(format!(
            "no force responds as {response}; acceptance is a force's response name"
        )));
    };
    let before = force.members.len();
    force.members.retain(|m| *m != want);
    if force.members.len() == before {
        return Verdict::Refused(refuse(format!(
            "force {response} reaches no {member_s}; acceptance is one of its members"
        )));
    }
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
    fn drop_member_leaves_one_member() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropMember("sum", "cli_b@1")) else {
            panic!("mutate");
        };
        let Subject::Contact(c) = &mutant else {
            panic!("contact mutant");
        };
        assert_ne!(hash(&c.coding), hash(&orig.coding));
        let members: Vec<String> = c.coding.forces[0]
            .members
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(members, ["cli_a@1"]);
        let Verdict::Refused(r) = contact_admission(&mutant) else {
            panic!("a dropped member must be refused");
        };
        assert_eq!(
            r.reason,
            "force sum has 1 members; its response takes 2. acceptance is 2 members (R84)"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropMember("sum", "cli_b@0")) else {
            panic!("missing member must refuse");
        };
        assert!(r.reason.contains("cli_b@0"), "{}", r.reason);
    }
}
