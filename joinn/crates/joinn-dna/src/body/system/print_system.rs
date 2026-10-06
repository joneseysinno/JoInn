//! `print_system`.

use std::fmt::Write as _;

use crate::body::system::SystemCoding;

/// Canonical system text, including the trailing newline. Bodies are sorted
/// by alias and forces by response name.
pub fn print_system(coding: &SystemCoding) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "codex {}", coding.codex);
    let _ = writeln!(out, "bodies");
    let mut bodies: Vec<_> = coding.bodies.iter().collect();
    bodies.sort_by(|a, b| a.alias.cmp(&b.alias));
    for body in bodies {
        let _ = writeln!(out, "contact:{} as {}", body.contact.to_hex(), body.alias);
    }
    let _ = writeln!(out, "forces");
    let mut forces: Vec<_> = coding.forces.iter().collect();
    forces.sort_by(|a, b| a.name.cmp(&b.name));
    for force in forces {
        let _ = writeln!(
            out,
            "{} {} cell:{} as {} on {}",
            force.kind.word(),
            force.frame,
            force.response.to_hex(),
            force.name,
            force.on
        );
    }
    match coding.lineage {
        Some(h) => {
            let _ = writeln!(out, "lineage {}", h.to_hex());
        }
        None => {
            let _ = writeln!(out, "lineage none");
        }
    }
    out
}
