//! The §2.14 catalogue on a contact: each mutant's admission, against what
//! §2.14 says it must answer.

use joinn_dna::{Cell, Contact, hash, print_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::check_contact;
use std::collections::BTreeMap;

use crate::fns::mutate::{Mutation, mutate};
use crate::fns::subject::Subject;

/// What a mutant must answer: admitted, or refused with a reason that holds
/// the given words.
enum Answer {
    Admitted,
    Refused(&'static str),
}

const CATALOGUE: [(Mutation, Answer); 6] = [
    (Mutation::DropForce("sum"), Answer::Admitted),
    (
        Mutation::SwapResponse(
            "sum",
            "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7",
        ),
        Answer::Refused("force sum pins cell:12b6e545"),
    ),
    (
        Mutation::ShiftMember("sum", "cli_a@1", 0),
        Answer::Refused("receptor: cli_a@0 "),
    ),
    (
        Mutation::DropMember("sum", "cli_b@1"),
        Answer::Refused("acceptance is 2 members (R84)"),
    ),
    (
        Mutation::DropGenome("cli_b"),
        Answer::Refused("member cli_b@1 names no instance"),
    ),
    (Mutation::RenameAlias("sum", "total"), Answer::Admitted),
];

/// One line per mutant: `Ok` when it answers as §2.14 says, `Err` otherwise.
pub(super) fn mutant_lines(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Vec<Result<String, String>> {
    let subject = Subject::Contact(contact.clone());
    let before = print_contact(&contact.coding);
    let mut lines = Vec::new();
    for (m, answer) in &CATALOGUE {
        let mutant = match mutate(&subject, m) {
            Verdict::Ok(Subject::Contact(c)) => c,
            Verdict::Ok(_) => {
                lines.push(Err(format!("{m:?}: the mutant is not a contact")));
                continue;
            }
            Verdict::Refused(r) => {
                lines.push(Err(format!("{m:?}: not applied: {}", r.reason)));
                continue;
            }
        };
        let after = print_contact(&mutant.coding);
        let line = match (check_contact(&mutant, cells, frames), answer) {
            (Verdict::Ok(()), Answer::Admitted) => match m {
                Mutation::DropForce(name) => {
                    let gone = mutant.coding.forces.iter().all(|f| f.name != *name)
                        && mutant
                            .coding
                            .genome
                            .iter()
                            .all(|g| g.instances.iter().all(|i| i != name));
                    if gone {
                        Ok(format!("{m:?}: admitted, {name} no longer exists"))
                    } else {
                        Err(format!("{m:?}: admitted, but {name} still exists"))
                    }
                }
                Mutation::RenameAlias(from, to) => {
                    if after == before.replace(from, to)
                        && hash(&mutant.coding) != hash(&contact.coding)
                    {
                        Ok(format!(
                            "{m:?}: admitted, canonical text differs only in {from} → {to}, hash {}…",
                            &hash(&mutant.coding).to_hex()[..8]
                        ))
                    } else {
                        Err(format!(
                            "{m:?}: admitted, but the canonical text moved beyond the name:\n{after}"
                        ))
                    }
                }
                _ => Ok(format!("{m:?}: admitted")),
            },
            (Verdict::Refused(r), Answer::Refused(words)) if r.reason.contains(words) => {
                Ok(format!("{m:?}: refused (ok): {}", r.reason))
            }
            (Verdict::Refused(r), _) => Err(format!("{m:?}: refused, against §2.14: {}", r.reason)),
            (Verdict::Ok(()), Answer::Refused(words)) => Err(format!(
                "{m:?}: admitted, against §2.14 (want a refusal holding `{words}`)"
            )),
        };
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::mutant_lines;
    use crate::fns::forces::corpus_cells;
    use joinn_dna::parse_contact;
    use joinn_frame::{FrameRegistry, Verdict};

    #[test]
    fn every_mutant_answers_as_the_catalogue_says() {
        let frames = FrameRegistry::phase1();
        let cells = corpus_cells(&frames).unwrap_or_else(|e| panic!("{e}"));
        let contact = match parse_contact(
            include_str!("../../../../corpus/phase7/calculator.contact"),
            &frames,
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let lines = mutant_lines(&contact, &cells, &frames);
        assert_eq!(lines.len(), 6);
        for line in &lines {
            assert!(line.is_ok(), "{line:?}");
        }
    }
}
