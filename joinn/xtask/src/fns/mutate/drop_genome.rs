//! Drop a genome instance (and its entry if emptied).

use joinn_dna::Body;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn drop_genome(body: &mut Body, instance: &str) -> Verdict<()> {
    let mut found = false;
    for entry in &mut body.coding.genome {
        let before = entry.instances.len();
        entry.instances.retain(|i| i != instance);
        if entry.instances.len() != before {
            found = true;
        }
    }
    body.coding.genome.retain(|e| !e.instances.is_empty());
    // Also drop wires that named the instance.
    body.coding
        .wires
        .retain(|w| w.src_instance != instance && w.dst_instance != instance);
    // Drop grant mentions.
    for insts in body.coding.grants.values_mut() {
        insts.retain(|i| i != instance);
    }
    body.coding.grants.retain(|_, insts| !insts.is_empty());
    if !found {
        return Verdict::Refused(refuse(format!(
            "no such instance {instance}; acceptance is a genome instance"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_calculator::load_calculator;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_dna::hash;
    use joinn_frame::Verdict;
    use joinn_link::membrane;
    use std::collections::BTreeSet;

    fn calculator() -> Subject {
        let src = include_str!("../../../../corpus/phase2/calculator.body");
        parse_subject("phase2/calculator.body", src)
            .unwrap_or_else(|e| panic!("parse calculator.body: {e}"))
    }

    #[test]
    fn drop_genome_removes_cli_b_from_membrane() {
        let s = calculator();
        let Subject::Body(orig) = &s else {
            panic!("body");
        };
        let orig_hash = hash(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropGenome("cli_b")) else {
            panic!("mutate");
        };
        let Subject::Body(body) = &mutant else {
            panic!("body mutant");
        };
        assert_ne!(hash(&body.coding), orig_hash);
        assert!(
            body.coding
                .genome
                .iter()
                .all(|e| !e.instances.iter().any(|i| i == "cli_b"))
        );
        let (_, cells) = load_calculator().unwrap_or_else(|e| panic!("load calculator cells: {e}"));
        let set = match membrane(body, &cells) {
            Verdict::Ok(set) => set,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let got: BTreeSet<String> = set.iter().map(|a| a.address.printed()).collect();
        assert!(
            !got.iter().any(|p| p.starts_with("cli_b@")),
            "cli_b must leave the membrane: {got:?}"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropGenome("ghost")) else {
            panic!("missing instance must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
