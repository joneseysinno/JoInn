//! Parse each control artifact by kind, apply declared opposition, then score.

use joinn_gate::GateItem;

use super::grade_opposed::grade_opposed;
use super::harness_fixtures::harness_fixtures;
use super::mutate::Mutation;
use super::subject::Subject;

pub(crate) fn run_gate_table(
    phase: &str,
    items: &[GateItem<Subject, Mutation>],
) -> Result<(u32, u32), String> {
    harness_fixtures()?;
    let mut failures = Vec::new();
    let mut rows = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let n = i + 1;
        match grade_opposed(n, item) {
            Ok(()) => rows.push((n, item.name, (item.check)())),
            Err(e) => failures.push(e),
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("\n"));
    }
    let mut n_ok = 0u32;
    let total = rows.len() as u32;
    for (i, name, ok) in rows {
        if ok {
            println!("{i} ok  {name}");
            n_ok += 1;
        } else {
            println!("{i} fail  {name}");
        }
    }
    println!("{phase}: {n_ok}/{total}");
    Ok((n_ok, total))
}

#[cfg(test)]
mod tests {
    use super::run_gate_table;
    use crate::fns::mutate::Mutation;
    use crate::fns::subject::Subject;
    use joinn_gate::GateItem;

    fn control_false(_: &Subject) -> bool {
        false
    }

    #[test]
    fn missing_control_artifact_names_the_item() {
        let err = run_gate_table(
            "phase test",
            &[GateItem {
                name: "probe",
                check: || true,
                control: control_false,
                control_artifact: "corpus/phase5/does-not-exist.txt",
                opposes: Mutation::Replace("x"),
            }],
        );
        match err {
            Ok(_) => panic!("missing artifact must refuse"),
            Err(msg) => {
                assert!(msg.contains("probe"), "{msg}");
                assert!(msg.contains("does-not-exist"), "{msg}");
            }
        }
    }

    #[test]
    fn unparseable_body_names_the_item_and_reason() {
        let err = run_gate_table(
            "phase test",
            &[GateItem {
                name: "bad universe",
                check: || true,
                control: control_false,
                control_artifact: "xtask/gate_fixtures/not_a_universe.universe",
                opposes: Mutation::DropLink("e0"),
            }],
        );
        match err {
            Ok(_) => panic!("unparseable artifact must refuse"),
            Err(msg) => {
                eprintln!("refusal: {msg}");
                assert!(msg.contains("bad universe"), "{msg}");
                assert!(
                    msg.contains("expected") || msg.contains("universe"),
                    "expected parser reason in: {msg}"
                );
            }
        }
    }
}
