//! Every non-legacy control against the catalogue mutations of its artifact kind.

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::{
        g5_assemble_control, g5_law4_control, g51_frame_control, g51_tails_control,
        g52_ids_control, g52_refusal_control, gate_five_items, gate_five_one_items,
        gate_five_two_items, resolve_named,
    };
    use joinn_frame::Verdict;
    use std::fs;

    fn mutations_for(subject: &Subject) -> Vec<Mutation> {
        match subject {
            Subject::Universe(_) => vec![
                Mutation::DropLink("e0"),
                Mutation::DropLink("path"),
                Mutation::FlipMark("e0", "calc.sum@2"),
                Mutation::ShiftPort("e0", "calc.sum@2", 9),
                Mutation::CorruptHash("calc"),
                Mutation::CorruptHash("units"),
                Mutation::CopyMember("function", "units", "calculation"),
                Mutation::DropLens("deployment"),
                Mutation::WireAcross("e0"),
                Mutation::DropGrant("e0"),
                Mutation::DropGrant("path"),
                Mutation::RenameLink("e0", "e1"),
                Mutation::RenameAlias("units", "meters"),
            ],
            Subject::Transcript(_) => vec![
                Mutation::DropLine(0),
                Mutation::DropLine(1),
                Mutation::DropLine(2),
                Mutation::SwapLines(0, 1),
                Mutation::SwapLines(2, 3),
            ],
            Subject::Text(_) => vec![
                Mutation::Replace("e0"),
                Mutation::Replace("units"),
                Mutation::Replace("link"),
            ],
            Subject::Body(_) | Subject::Lock(_) => Vec::new(),
        }
    }

    #[test]
    fn controls_cross_matrix() {
        let tables = [
            gate_five_items(),
            gate_five_one_items(),
            gate_five_two_items(),
        ];
        for table in tables {
            for item in table {
                let path = resolve_named(0, item.name, item.control_artifact)
                    .unwrap_or_else(|e| panic!("{e}"));
                let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
                let subject = parse_subject(item.control_artifact, &text)
                    .unwrap_or_else(|e| panic!("{}: {e}", item.name));
                let mut fired = Vec::new();
                for mutation in mutations_for(&subject) {
                    let Verdict::Ok(mutant) = mutate(&subject, &mutation) else {
                        continue;
                    };
                    if (item.control)(&mutant) {
                        fired.push(format!("{mutation:?}"));
                    }
                }
                println!("{}: {}", item.name, fired.join(", "));
            }
        }

        let path = resolve_named(0, "universe", "corpus/phase5/universe.universe")
            .unwrap_or_else(|e| panic!("{e}"));
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        let universe = parse_subject("corpus/phase5/universe.universe", &text)
            .unwrap_or_else(|e| panic!("{e}"));
        let quiet = [
            Mutation::CorruptHash("calc"),
            Mutation::CorruptHash("units"),
            Mutation::RenameLink("e0", "e1"),
            Mutation::RenameAlias("units", "meters"),
        ];
        type Control = fn(&Subject) -> bool;
        let controls: &[(&str, Control)] = &[
            ("g52_ids_control", g52_ids_control),
            ("g52_refusal_control", g52_refusal_control),
            ("g5_assemble_control", g5_assemble_control),
            ("g51_tails_control", g51_tails_control),
            ("g51_frame_control", g51_frame_control),
            ("g5_law4_control", g5_law4_control),
        ];
        for mutation in quiet {
            let Verdict::Ok(mutant) = mutate(&universe, &mutation) else {
                panic!("mutate {mutation:?} must produce a subject");
            };
            for (name, control) in controls {
                assert!(!control(&mutant), "{name} answered true on {mutation:?}");
            }
        }
    }
}
