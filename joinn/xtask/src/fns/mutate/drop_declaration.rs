//! Drop the `assert H₁ = 0` line from a body or a universe.

use joinn_dna::Assertion;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn drop_declaration(declarations: &mut Vec<Assertion>) -> Verdict<()> {
    if !declarations.contains(&Assertion::H1Zero) {
        return Verdict::Refused(refuse(
            "no declaration to drop; acceptance is a subject that declares assert H₁ = 0",
        ));
    }
    declarations.retain(|d| *d != Assertion::H1Zero);
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::corpus_fixture::corpus_fixture;
    use crate::fns::mutate::{Mutation, mutate, reprint};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::workspace_root::workspace_root;
    use joinn_dna::{Assertion, hash};
    use joinn_frame::Verdict;
    use joinn_link::{BodyStore, assemble_universe, bind};
    use std::fs;

    const CLI_INPUT_OPEN: &str = "eb8479f42622365a12d5eb6ae3f35386d782cb9679144582774c49ea83945678";
    const FMT_TWIN: &str = "f1b05fd17e9eeaa284a03a6ea586073b7b328e377c169628fa347cdec239b0cf";

    fn read(rel: &str) -> Subject {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let text =
            fs::read_to_string(root.join("corpus").join(rel)).unwrap_or_else(|e| panic!("{e}"));
        parse_subject(rel, &text).unwrap_or_else(|e| panic!("{e}"))
    }

    #[test]
    fn a_dropped_body_declaration_lets_the_swap_insert() {
        let (cells, _) = corpus_fixture(&[]);
        let Subject::Body(mut calc) = read("phase2/calculator.body") else {
            panic!("body");
        };
        calc.coding.declarations = vec![Assertion::H1Zero];
        let swapped = match mutate(
            &Subject::Body(calc),
            &Mutation::SwapCell("cli_b", CLI_INPUT_OPEN),
        ) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let Subject::Body(declared) = &swapped else {
            panic!("body");
        };
        match BodyStore::new().insert(declared.clone(), cells.clone(), "declared swap") {
            Verdict::Refused(r) => assert!(r.reason.contains("assert H₁ = 0"), "{}", r.reason),
            Verdict::Ok(_) => panic!("the declared swap must be refused"),
        }
        let dropped = match mutate(&swapped, &Mutation::DropDeclaration) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert!(!reprint(&dropped).contains("declarations"));
        let Subject::Body(body) = &dropped else {
            panic!("body");
        };
        assert_ne!(hash(&body.coding), hash(&declared.coding));
        match BodyStore::new().insert(body.clone(), cells, "dropped") {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("after DropDeclaration it must insert: {}", r.reason),
        }
        let Verdict::Refused(r) =
            mutate(&read("phase2/calculator.body"), &Mutation::DropDeclaration)
        else {
            panic!("a body with no declaration must refuse DropDeclaration");
        };
        assert!(r.reason.contains("no declaration to drop"), "{}", r.reason);
    }

    #[test]
    fn a_dropped_universe_declaration_lets_the_twin_assemble() {
        let (_, store) = corpus_fixture(&[
            "phase2/calculator.body",
            "phase4/fmt.body",
            "phase4/fmt_twin.body",
        ]);
        let assemble = |s: &Subject| {
            let Subject::Universe(u) = s else {
                panic!("universe");
            };
            let bound = match bind(u, &store) {
                Verdict::Ok(b) => b,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assemble_universe(u, &bound)
        };
        let twin = match mutate(
            &read("phase4/loop_declared.universe"),
            &Mutation::SwapBinding("fmt", FMT_TWIN),
        ) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match assemble(&twin) {
            Verdict::Refused(r) => assert!(r.reason.contains("assert H₁ = 0"), "{}", r.reason),
            Verdict::Ok(()) => panic!("the declared fmt_twin mutant must be refused"),
        }
        let dropped = match mutate(&twin, &Mutation::DropDeclaration) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert!(!reprint(&dropped).contains("declarations"));
        match assemble(&dropped) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("after DropDeclaration it must assemble: {}", r.reason),
        }
        let Verdict::Refused(r) = mutate(&read("phase4/loop.universe"), &Mutation::DropDeclaration)
        else {
            panic!("a universe with no declaration must refuse DropDeclaration");
        };
        assert!(r.reason.contains("no declaration to drop"), "{}", r.reason);
    }
}
