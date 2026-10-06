//! Drop the system force whose response is named.

use joinn_frame::Verdict;

use crate::fns::subject::SystemSubject;

use super::refuse::refuse;

pub(super) fn drop_system_force(s: &mut SystemSubject, response: &str) -> Verdict<()> {
    let forces = &mut s.system.coding.forces;
    let before = forces.len();
    forces.retain(|f| f.name != response);
    if forces.len() == before {
        return Verdict::Refused(refuse(format!(
            "no force responds as {response}; acceptance is a force's response name"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::Mutation;
    use crate::fns::mutate::apply::apply;
    use crate::fns::mutate::system_fixture::system_fixture;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    #[test]
    fn drop_force_leaves_a_system_with_no_force() {
        let mut s = system_fixture();
        let applied = apply(&mut s, &Mutation::DropForce("count"));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let Subject::System(sys) = &s else {
            panic!("system");
        };
        assert!(sys.system.coding.forces.is_empty());
        assert_eq!(sys.system.coding.bodies.len(), 1);
        let Verdict::Refused(r) = apply(&mut s, &Mutation::DropForce("count")) else {
            panic!("a missing force must refuse");
        };
        assert!(r.reason.contains("count"), "{}", r.reason);
    }
}
