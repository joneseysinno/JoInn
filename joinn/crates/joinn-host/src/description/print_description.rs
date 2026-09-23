//! Canonical description text, including the trailing newline.

use std::fmt::Write as _;

use joinn_dna::Direction;

use super::Description;
use super::quoted::quoted;

/// Canonical description text, including the trailing newline.
pub fn print_description(d: &Description) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "description {{");
    let _ = writeln!(out, "  cell {}", d.cell.to_hex());
    let _ = writeln!(out, "  instance {}", d.instance);
    let _ = writeln!(out, "  label {}", quoted(&d.label));
    let _ = writeln!(out, "  role {}", d.role.as_str());
    for p in &d.ports {
        let dir = match p.direction {
            Direction::In => "in",
            Direction::Out => "out",
        };
        let _ = write!(
            out,
            "  port {} {dir} {} {} label {} role {}",
            p.position,
            p.frame,
            quoted(&p.name),
            quoted(&p.label),
            p.role.as_str()
        );
        if let Some(v) = &p.value {
            let _ = write!(out, " value {v}");
        }
        out.push('\n');
    }
    let _ = writeln!(out, "}}");
    out
}
