//! Drop a declared grant for a link.
//!
//! Snag (P52-04): universe grants land in P52-11. Until then there is no grants
//! field on `Universe`, so this always refuses naming the link.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn drop_grant(_u: &mut Universe, link: &str) -> Verdict<()> {
    Verdict::Refused(refuse(format!(
        "no grant on link {link}; acceptance is a declared grants section (P52-11)"
    )))
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn drop_grant_refuses_naming_link_until_p52_11() {
        let s = universe();
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropGrant("e0")) else {
            panic!("DropGrant must refuse before grants exist");
        };
        assert!(r.reason.contains("e0"), "{}", r.reason);
        assert!(r.reason.contains("grant"), "{}", r.reason);
        let Verdict::Refused(r2) = mutate(&s, &Mutation::DropGrant("ghost")) else {
            panic!("missing grant target must refuse");
        };
        assert!(r2.reason.contains("ghost"), "{}", r2.reason);
    }
}
