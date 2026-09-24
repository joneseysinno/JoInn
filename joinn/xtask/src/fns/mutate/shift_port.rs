//! Shift a member's port index.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::parse_member_ref::parse_member_ref;
use super::refuse::refuse;

pub(super) fn shift_port(
    u: &mut Universe,
    link_id: &str,
    member_s: &str,
    to: u32,
) -> Verdict<()> {
    let want = match parse_member_ref(member_s) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let Some(link) = u.coding.links.iter_mut().find(|l| l.id == link_id) else {
        return Verdict::Refused(refuse(format!(
            "no such link {link_id}; acceptance is a declared link id"
        )));
    };
    let Some(member) = link.members.iter_mut().find(|m| {
        m.body == want.body && m.instance == want.instance && m.port == want.port
    }) else {
        return Verdict::Refused(refuse(format!(
            "no such member {member_s} on link {link_id}; acceptance is a declared member"
        )));
    };
    member.port = to;
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_phase5_bodies::load_phase5_bodies;
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;
    use joinn_link::{assemble_universe, bind_bodies, hash_universe};

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn shift_port_assembly_names_no_such_port() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::ShiftPort("e0", "calc.sum@2", 9)) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        let supplied = load_phase5_bodies().expect("bodies");
        let bound = match bind_bodies(u, &supplied) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match assemble_universe(u, &bound) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("no such port"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("assembly must refuse shifted port"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::ShiftPort("ghost", "calc.sum@2", 9)) else {
            panic!("missing link must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
