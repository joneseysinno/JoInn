//! Rename a body alias everywhere it appears in the universe.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn rename_alias(u: &mut Universe, from: &str, to: &str) -> Verdict<()> {
    if !u.coding.bodies.iter().any(|b| b.alias == from) {
        return Verdict::Refused(refuse(format!(
            "no such alias {from}; acceptance is a declared body alias"
        )));
    }
    if u.coding.bodies.iter().any(|b| b.alias == to) {
        return Verdict::Refused(refuse(format!(
            "alias {to} already exists; acceptance is a fresh alias"
        )));
    }
    for binding in &mut u.coding.bodies {
        if binding.alias == from {
            binding.alias = to.to_owned();
        }
    }
    for link in &mut u.coding.links {
        for member in &mut link.members {
            if member.body == from {
                member.body = to.to_owned();
            }
        }
    }
    for wire in &mut u.coding.cross_wires {
        if wire.src_body == from {
            wire.src_body = to.to_owned();
        }
        if wire.dst_body == from {
            wire.dst_body = to.to_owned();
        }
    }
    for lens in &mut u.coding.lenses {
        for galaxy in &mut lens.galaxies {
            for system in &mut galaxy.systems {
                for body in &mut system.bodies {
                    if body == from {
                        *body = to.to_owned();
                    }
                }
            }
        }
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
    fn rename_alias_removes_units_name() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::RenameAlias("units", "meters")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(u.coding.bodies.iter().all(|b| b.alias != "units"));
        assert!(u.coding.bodies.iter().any(|b| b.alias == "meters"));
        // Runtime still injects under "units", so the renamed body never fires.
        match units_after(u, Some("e0")) {
            Ok((0, _)) => {}
            Err(_) => {}
            Ok((n, _)) => panic!("units must not fire after RenameAlias, got {n}"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::RenameAlias("ghost", "meters")) else {
            panic!("missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
