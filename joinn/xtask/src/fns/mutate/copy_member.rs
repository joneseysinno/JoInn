//! Copy a body alias into another system of the same lens.

use joinn_frame::Verdict;
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn copy_member(
    u: &mut Universe,
    lens_name: &str,
    alias: &str,
    into_system: &str,
) -> Verdict<()> {
    let Some(lens) = u.coding.lenses.iter_mut().find(|l| l.name == lens_name) else {
        return Verdict::Refused(refuse(format!(
            "no such lens {lens_name}; acceptance is a declared lens"
        )));
    };
    let mut found_alias = false;
    for galaxy in &lens.galaxies {
        for system in &galaxy.systems {
            if system.bodies.iter().any(|b| b == alias) {
                found_alias = true;
            }
        }
    }
    if !found_alias {
        return Verdict::Refused(refuse(format!(
            "no such alias {alias} in lens {lens_name}; acceptance is a body placed in that lens"
        )));
    }
    for galaxy in &mut lens.galaxies {
        if let Some(system) = galaxy.systems.iter_mut().find(|s| s.name == into_system) {
            if !system.bodies.iter().any(|b| b == alias) {
                system.bodies.push(alias.to_owned());
            }
            return Verdict::Ok(());
        }
    }
    Verdict::Refused(refuse(format!(
        "no such system {into_system} in lens {lens_name}; acceptance is a declared system"
    )))
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;
    use joinn_link::{check_lenses, hash_universe};

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn copy_member_exclusivity_refuses_units() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) =
            mutate(&s, &Mutation::CopyMember("function", "units", "calculation"))
        else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        match check_lenses(u) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("units"), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("exclusivity must refuse copied member"),
        }
        let Verdict::Refused(r) =
            mutate(&s, &Mutation::CopyMember("function", "ghost", "calculation"))
        else {
            panic!("missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
