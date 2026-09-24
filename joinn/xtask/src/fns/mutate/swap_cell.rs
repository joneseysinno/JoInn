//! Swap the cell hash bound to a genome instance.

use joinn_dna::{Body, GenomeTarget};
use joinn_frame::{Hash, Verdict};

use super::refuse::refuse;

pub(super) fn swap_cell(body: &mut Body, instance: &str, to_hash: &str) -> Verdict<()> {
    let Some(hash) = Hash::parse_hex(to_hash) else {
        return Verdict::Refused(refuse(format!(
            "hash {to_hash} is not a cell hash; acceptance is 64 hex digits"
        )));
    };
    let mut found = false;
    for entry in &mut body.coding.genome {
        if entry.instances.iter().any(|i| i == instance) {
            // Split the instance onto its own entry with the new cell.
            entry.instances.retain(|i| i != instance);
            found = true;
        }
    }
    if !found {
        return Verdict::Refused(refuse(format!(
            "no such instance {instance}; acceptance is a genome instance"
        )));
    }
    body.coding.genome.retain(|e| !e.instances.is_empty());
    body.coding.genome.push(joinn_dna::GenomeEntry {
        target: GenomeTarget::Cell(hash),
        instances: vec![instance.to_owned()],
    });
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_calculator::load_calculator;
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_dna::{hash, GenomeTarget};
    use joinn_frame::Verdict;
    use joinn_link::membrane;

    fn calculator() -> Subject {
        let src = include_str!("../../../../corpus/phase2/calculator.body");
        parse_subject("phase2/calculator.body", src).expect("parse")
    }

    #[test]
    fn swap_cell_makes_membrane_refuse_or_change_hash() {
        let s = calculator();
        let Subject::Body(orig) = &s else {
            panic!("body");
        };
        let orig_hash = hash(&orig.coding);
        // Use the format cell hash (not sum) so sum's contract disappears.
        let (_, cells) = load_calculator().expect("cells");
        let format_hex = cells
            .keys()
            .find(|h| {
                // pick a hash that is not currently sum's
                !orig.coding.genome.iter().any(|e| {
                    matches!(&e.target, GenomeTarget::Cell(c) if c == *h)
                        && e.instances.iter().any(|i| i == "sum")
                })
            })
            .map(|h| h.to_hex())
            .expect("another cell");
        let format_static: &'static str = Box::leak(format_hex.into_boxed_str());
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SwapCell("sum", format_static)) else {
            panic!("mutate");
        };
        let Subject::Body(body) = &mutant else {
            panic!("body mutant");
        };
        assert_ne!(hash(&body.coding), orig_hash);
        let sum_target = body
            .coding
            .genome
            .iter()
            .find(|e| e.instances.iter().any(|i| i == "sum"))
            .map(|e| &e.target);
        assert!(
            matches!(sum_target, Some(GenomeTarget::Cell(h)) if h.to_hex() == format_static),
            "sum must point at the swapped cell"
        );
        // Downstream: membrane either refuses (wrong ports) or no longer has sum@2 alone as before.
        match membrane(body, &cells) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("sum") || r.reason.contains("cell"), "{}", r.reason);
            }
            Verdict::Ok(set) => {
                let ports: Vec<_> = set.iter().map(|a| a.address.printed()).collect();
                assert!(
                    !ports.iter().any(|p| p == "sum@2")
                        || ports != ["cli_a@0", "cli_b@0", "sum@2"],
                    "membrane must reflect the swapped cell: {ports:?}"
                );
            }
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::SwapCell("ghost", format_static)) else {
            panic!("missing instance must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
