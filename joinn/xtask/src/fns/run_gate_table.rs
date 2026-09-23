//! Read each control artifact, require damage to flip the answer, then run the table.

use joinn_gate::{Artifact, GateItem, run_opposed};
use std::fs;

use super::{damage_bytes, resolve_named};

pub(crate) fn run_gate_table(phase: &str, items: &[GateItem]) -> Result<(u32, u32), String> {
    let mut loaded = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let path = resolve_named(i + 1, item.name, item.control_artifact)?;
        let bytes = fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        loaded.push(bytes);
    }
    let mut blind = Vec::new();
    for (item, bytes) in items.iter().zip(loaded.iter()) {
        let real = Artifact {
            path: item.control_artifact,
            bytes,
        };
        let damaged_buf = damage_bytes(bytes);
        let damaged = Artifact {
            path: item.control_artifact,
            bytes: &damaged_buf,
        };
        if (item.control)(&real) == (item.control)(&damaged) {
            blind.push(item.name);
        }
    }
    if !blind.is_empty() {
        return Err(format!(
            "control does not read its artifact: {}",
            blind.join("; ")
        ));
    }
    let refs: Vec<&[u8]> = loaded.iter().map(Vec::as_slice).collect();
    let rows = run_opposed(items, &refs)?;
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

    fn control_false(_: &joinn_gate::Artifact) -> bool {
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
    fn insensitive_fixture_is_refused() {
        fn ignores(_: &joinn_gate::Artifact) -> bool {
            false
        }
        let err = run_gate_table(
            "phase fixture",
            &[GateItem {
                name: "ignores its bytes",
                check: || true,
                control: ignores,
                control_artifact: "xtask/gate_fixtures/insensitive.rs",
            }],
        );
        match err {
            Ok(_) => panic!("an ignoring control must refuse the run"),
            Err(msg) => {
                assert!(msg.contains("ignores its bytes"), "{msg}");
                assert!(msg.contains("does not read its artifact"), "{msg}");
            }
        }
    }
}
