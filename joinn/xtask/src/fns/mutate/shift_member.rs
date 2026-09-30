//! Move one member of a force to another port of the same instance.

use joinn_dna::Contact;
use joinn_frame::Verdict;

use super::parse_contact_member::parse_contact_member;
use super::refuse::refuse;

pub(super) fn shift_member(
    contact: &mut Contact,
    response: &str,
    member_s: &str,
    to: u32,
) -> Verdict<()> {
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
    let Some(member) = force.members.iter_mut().find(|m| **m == want) else {
        return Verdict::Refused(refuse(format!(
            "force {response} reaches no {member_s}; acceptance is one of its members"
        )));
    };
    member.port = to;
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
    fn shift_member_moves_cli_a_to_its_in_port() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::ShiftMember("sum", "cli_a@1", 0)) else {
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
        assert_eq!(members, ["cli_a@0", "cli_b@1"]);
        let Verdict::Refused(r) = contact_admission(&mutant) else {
            panic!("a shifted member must be refused");
        };
        assert_eq!(
            r.reason,
            "receptor: cli_a@0 is Text 1, the force is combine ℤ 1; acceptance is a member in ℤ 1"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::ShiftMember("sum", "ghost@1", 0)) else {
            panic!("missing member must refuse");
        };
        assert!(r.reason.contains("ghost@1"), "{}", r.reason);
    }
}
