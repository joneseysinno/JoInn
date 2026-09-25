//! Flip a member's Tail/Head mark.

use joinn_frame::Verdict;
use joinn_link::{Mark, Universe};

use super::parse_member_ref::parse_member_ref;
use super::refuse::refuse;

pub(super) fn flip_mark(u: &mut Universe, link_id: &str, member_s: &str) -> Verdict<()> {
    let want = match parse_member_ref(member_s) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let Some(link) = u.coding.links.iter_mut().find(|l| l.id == link_id) else {
        return Verdict::Refused(refuse(format!(
            "no such link {link_id}; acceptance is a declared link id"
        )));
    };
    let Some(member) = link
        .members
        .iter_mut()
        .find(|m| m.body == want.body && m.instance == want.instance && m.port == want.port)
    else {
        return Verdict::Refused(refuse(format!(
            "no such member {member_s} on link {link_id}; acceptance is a declared member"
        )));
    };
    member.mark = match member.mark {
        Mark::Tail => Mark::Head,
        Mark::Head => Mark::Tail,
        Mark::None => Mark::Tail,
    };
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_phase5_bodies::load_phase5_bodies;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;
    use joinn_link::{bind_bodies, check_link_types, hash_universe};

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn flip_mark_makes_typing_refuse() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::FlipMark("e0", "calc.sum@2")) else {
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
        match check_link_types(u, &bound) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("calc.sum@2"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("typing must refuse flipped mark"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::FlipMark("e0", "calc.sum@9")) else {
            panic!("missing member must refuse");
        };
        assert!(r.reason.contains("calc.sum@9"), "{}", r.reason);
    }
}
