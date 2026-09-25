//! Drop a lens by name.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn drop_lens(u: &mut Universe, name: &str) -> Verdict<()> {
    let before = u.coding.lenses.len();
    u.coding.lenses.retain(|l| l.name != name);
    if u.coding.lenses.len() == before {
        return Verdict::Refused(refuse(format!(
            "no such lens {name}; acceptance is a declared lens"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;
    use joinn_link::hash_universe;
    use std::collections::BTreeSet;

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src)
            .unwrap_or_else(|e| panic!("parse universe: {e}"))
    }

    fn systems_placing_units(u: &joinn_link::Universe) -> BTreeSet<String> {
        let mut systems = BTreeSet::new();
        for lens in &u.coding.lenses {
            for galaxy in &lens.galaxies {
                for system in &galaxy.systems {
                    if system.bodies.iter().any(|a| a == "units") {
                        systems.insert(system.name.clone());
                    }
                }
            }
        }
        systems
    }

    #[test]
    fn drop_lens_leaves_units_in_one_system() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        assert_eq!(systems_placing_units(orig).len(), 2);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropLens("deployment")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(
            systems_placing_units(u).len() < 2,
            "fewer than two lenses must place units"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropLens("ghost")) else {
            panic!("missing lens must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
