//! Drop a declared grant for a link.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn drop_grant(u: &mut Universe, link: &str) -> Verdict<()> {
    if u.coding.grants.remove(link).is_none() {
        return Verdict::Refused(refuse(format!(
            "no grant on link {link}; acceptance is a declared grant on that link"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::units_after::units_after;
    use joinn_frame::Verdict;
    use joinn_link::hash_universe;

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src)
            .unwrap_or_else(|e| panic!("parse universe: {e}"))
    }

    #[test]
    fn drop_grant_e0_stops_units() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropGrant("e0")) else {
            panic!("DropGrant e0 must accept");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(!u.coding.grants.contains_key("e0"));
        match units_after(u) {
            Ok((0, _)) => {}
            Ok((n, _)) => panic!("units must not fire without grant on e0, got {n}"),
            Err(reason) => panic!("{reason}"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropGrant("ghost")) else {
            panic!("missing grant target must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
