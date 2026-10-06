//! `print_contact`.

use joinn_frame::Hash;
use std::fmt::Write as _;

use crate::body::contact::ContactCoding;

/// Canonical contact text, including the trailing newline. Forces are sorted
/// by response name and members by (instance, port): member order is never
/// hashed. `grows` is printed only when present, so a contact without it
/// prints as before.
pub fn print_contact(coding: &ContactCoding) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "budget");
    let _ = writeln!(out, "steps {}", coding.budget_steps);
    let _ = writeln!(out, "codex {}", coding.codex);
    let _ = writeln!(out, "genome");
    let mut pairs: Vec<(Hash, &str)> = coding
        .genome
        .iter()
        .flat_map(|g| g.instances.iter().map(move |i| (g.cell, i.as_str())))
        .collect();
    pairs.sort();
    let mut grouped: Vec<(Hash, Vec<&str>)> = Vec::new();
    for (h, inst) in pairs {
        match grouped.last_mut() {
            Some((gh, list)) if *gh == h => list.push(inst),
            _ => grouped.push((h, vec![inst])),
        }
    }
    for (h, insts) in grouped {
        let _ = writeln!(out, "cell:{} as {}", h.to_hex(), insts.join(", "));
    }
    let _ = writeln!(out, "grants");
    for (cap, insts) in &coding.grants {
        let _ = writeln!(out, "{cap} {}", insts.join(" "));
    }
    if !coding.reads.is_empty() {
        let _ = writeln!(out, "read");
        for name in &coding.reads {
            let _ = writeln!(out, "{name}");
        }
    }
    match coding.lineage {
        Some(h) => {
            let _ = writeln!(out, "lineage {}", h.to_hex());
        }
        None => {
            let _ = writeln!(out, "lineage none");
        }
    }
    let _ = writeln!(out, "forces");
    let mut forces: Vec<_> = coding.forces.iter().collect();
    forces.sort_by(|a, b| a.name.cmp(&b.name));
    for force in forces {
        let mut members = force.members.clone();
        members.sort();
        let members: Vec<String> = members.iter().map(ToString::to_string).collect();
        let _ = writeln!(
            out,
            "{} {} cell:{} as {} from {}",
            force.kind.word(),
            force.frame,
            force.response.to_hex(),
            force.name,
            members.join(" ")
        );
    }
    if let Some(grows) = &coding.grows {
        let _ = writeln!(out, "grows");
        let _ = writeln!(
            out,
            "cell:{} as {} accepts {}",
            grows.cell.to_hex(),
            grows.name,
            grows.accepts.word()
        );
    }
    out
}
