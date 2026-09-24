//! Rename a link id, and any regulatory keys that used it.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn rename_link(u: &mut Universe, from: &str, to: &str) -> Verdict<()> {
    if u.coding.links.iter().any(|l| l.id == to) {
        return Verdict::Refused(refuse(format!(
            "link {to} already exists; acceptance is a fresh link id"
        )));
    }
    let mut found = false;
    for link in &mut u.coding.links {
        if link.id == from {
            link.id = to.to_owned();
            found = true;
            break;
        }
    }
    if !found {
        return Verdict::Refused(refuse(format!(
            "no such link {from}; acceptance is a declared link id"
        )));
    }
    if let Some(v) = u.regulatory.names.remove(from) {
        u.regulatory.names.insert(to.to_owned(), v);
    }
    if let Some(v) = u.regulatory.labels.remove(from) {
        u.regulatory.labels.insert(to.to_owned(), v);
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
    fn rename_link_removes_e0_id() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::RenameLink("e0", "e1")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(u.coding.links.iter().all(|l| l.id != "e0"));
        assert!(u.coding.links.iter().any(|l| l.id == "e1"));
        // Grant still names e0 at runtime, so the renamed link is not granted.
        match units_after(u, true) {
            Ok((0, _)) => {}
            Err(reason) => assert!(
                reason.contains("e0") || reason.contains("link"),
                "{reason}"
            ),
            Ok((n, _)) => panic!("units must not fire after RenameLink, got {n}"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::RenameLink("ghost", "e1")) else {
            panic!("missing link must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
