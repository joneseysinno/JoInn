//! Parse each control artifact by kind, then run the opposed table.

use joinn_gate::{GateItem, run_opposed};
use std::fs;

use super::parse_subject::parse_subject;
use super::resolve_named;
use super::subject::Subject;

pub(crate) fn run_gate_table(
    phase: &str,
    items: &[GateItem<Subject>],
) -> Result<(u32, u32), String> {
    let mut subjects = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let path = resolve_named(i + 1, item.name, item.control_artifact)?;
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let subject = parse_subject(item.control_artifact, &text).map_err(|reason| {
            format!(
                "gate item {} ({}): {reason}",
                i + 1,
                item.name
            )
        })?;
        subjects.push(subject);
    }
    let rows = run_opposed(items, &subjects)?;
    let mut n = 0u32;
    let total = rows.len() as u32;
    for (i, name, ok) in rows {
        if ok {
            println!("{i} ok  {name}");
            n += 1;
        } else {
            println!("{i} fail  {name}");
        }
    }
    println!("{phase}: {n}/{total}");
    Ok((n, total))
}

#[cfg(test)]
mod tests {
    use super::run_gate_table;
    use joinn_gate::GateItem;
    use crate::fns::subject::Subject;

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
