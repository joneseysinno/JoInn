//! Re-pin a force's response to another coding hash.

use joinn_dna::Contact;
use joinn_frame::{Hash, Verdict};

use super::refuse::refuse;

pub(super) fn swap_response(contact: &mut Contact, response: &str, to_hash: &str) -> Verdict<()> {
    let Some(hash) = Hash::parse_hex(to_hash) else {
        return Verdict::Refused(refuse(format!(
            "hash {to_hash} is not a cell hash; acceptance is 64 hex digits"
        )));
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
    force.response = hash;
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

    const MUL: &str = "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7";

    #[test]
    fn swap_response_pins_mul() {
        let s = contact_fixture();
        let Subject::Contact(orig) = &s else {
            panic!("contact");
        };
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SwapResponse("sum", MUL)) else {
            panic!("mutate");
        };
        let Subject::Contact(c) = &mutant else {
            panic!("contact mutant");
        };
        assert_ne!(hash(&c.coding), hash(&orig.coding));
        assert_eq!(c.coding.forces.len(), 1);
        assert_eq!(c.coding.forces[0].response.to_hex(), MUL);
        let Verdict::Refused(r) = contact_admission(&mutant) else {
            panic!("a re-pinned response must be refused");
        };
        assert_eq!(
            r.reason,
            format!(
                "combine on ℤ 1 responds by cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39; force sum pins cell:{MUL}. acceptance is the registered response"
            )
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::SwapResponse("ghost", MUL)) else {
            panic!("missing force must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
