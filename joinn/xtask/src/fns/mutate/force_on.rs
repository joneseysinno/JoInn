//! Point a system force at another body alias.

use joinn_frame::Verdict;

use crate::fns::subject::SystemSubject;

use super::refuse::refuse;

pub(super) fn force_on(s: &mut SystemSubject, response: &str, alias: &str) -> Verdict<()> {
    let Some(force) = s
        .system
        .coding
        .forces
        .iter_mut()
        .find(|f| f.name == response)
    else {
        return Verdict::Refused(refuse(format!(
            "no force responds as {response}; acceptance is a force's response name"
        )));
    };
    if force.on == alias {
        return Verdict::Refused(refuse(format!(
            "{response} is already on {alias}; acceptance is another alias"
        )));
    }
    force.on = alias.to_owned();
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
    fn force_on_moves_the_force_and_nothing_else() {
        let before = system_fixture();
        let mut s = before.clone();
        let applied = apply(&mut s, &Mutation::ForceOn("count", "ghost"));
        assert!(matches!(applied, Verdict::Ok(())), "{applied:?}");
        let (Subject::System(a), Subject::System(b)) = (&before, &s) else {
            panic!("system");
        };
        assert_eq!(b.system.coding.forces[0].on, "ghost");
        assert_eq!(a.system.coding.bodies, b.system.coding.bodies);
        assert_eq!(
            a.system.coding.forces[0].name,
            b.system.coding.forces[0].name
        );
        let Verdict::Refused(r) = apply(&mut s, &Mutation::ForceOn("count", "ghost")) else {
            panic!("the same alias must refuse");
        };
        assert!(r.reason.contains("already on ghost"), "{}", r.reason);
        let Verdict::Refused(r) = apply(&mut s, &Mutation::ForceOn("total", "numbers")) else {
            panic!("a missing force must refuse");
        };
        assert!(r.reason.contains("total"), "{}", r.reason);
    }
}
