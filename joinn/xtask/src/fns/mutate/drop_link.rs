//! Drop a link by id.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn drop_link(u: &mut Universe, id: &str) -> Verdict<()> {
    let before = u.coding.links.len();
    u.coding.links.retain(|l| l.id != id);
    if u.coding.links.len() == before {
        return Verdict::Refused(refuse(format!(
            "no such link {id}; acceptance is a declared link id"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::units_after::units_after;
    use joinn_frame::Verdict;
    use joinn_link::hash_universe;

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn drop_link_e0_stops_units() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropLink("e0")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(u.coding.links.iter().all(|l| l.id != "e0"));
        match units_after(u, Some("e0")) {
            Ok((0, _)) => {}
            Err(reason) => assert!(
                reason.contains("e0") || reason.contains("link"),
                "{reason}"
            ),
            Ok((n, _)) => panic!("units must not fire without e0, got {n}"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropLink("missing_link")) else {
            panic!("missing target must refuse");
        };
        assert!(r.reason.contains("missing_link"), "{}", r.reason);
    }
}
