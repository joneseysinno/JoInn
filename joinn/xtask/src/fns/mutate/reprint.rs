//! Canonical text for a subject, ready to re-parse.

use crate::fns::subject::Subject;
use joinn_dna::{print_body, print_contact};
use joinn_link::print_universe;
use std::fmt::Write as _;

use super::reprint_regulatory::reprint_regulatory;

pub(crate) fn reprint(s: &Subject) -> String {
    match s {
        Subject::Body(body) => {
            let mut out = print_body(&body.coding);
            out.push_str(&reprint_regulatory(&body.regulatory));
            out
        }
        Subject::Contact(contact) => {
            let mut out = print_contact(&contact.coding);
            out.push_str(&reprint_regulatory(&contact.regulatory));
            out
        }
        Subject::System(_) => String::new(),
        Subject::Universe(u) => {
            let mut out = print_universe(&u.coding);
            if !u.regulatory.names.is_empty() || !u.regulatory.labels.is_empty() {
                out.push_str("---\nregulatory {\n");
                if !u.regulatory.names.is_empty() {
                    out.push_str("  names {");
                    for (k, v) in &u.regulatory.names {
                        let _ = write!(out, " {k} \"{v}\"");
                    }
                    out.push_str(" }\n");
                }
                if !u.regulatory.labels.is_empty() {
                    out.push_str("  labels {");
                    for (k, v) in &u.regulatory.labels {
                        let _ = write!(out, " {k} \"{v}\"");
                    }
                    out.push_str(" }\n");
                }
                out.push_str("}\n");
            }
            out
        }
        Subject::Lock(rows) => {
            let mut out = String::new();
            for row in rows {
                if row.phase == "phase 0" && row.n == 1 && row.total == 1 && !row.legacy {
                    out.push_str("phase 0: pass\n");
                } else if row.phase == "phase 0" && row.n == 0 && row.total == 1 && !row.legacy {
                    out.push_str("phase 0: fail\n");
                } else if row.legacy {
                    let _ = writeln!(out, "{}: {}/{} legacy", row.phase, row.n, row.total);
                } else {
                    let _ = writeln!(out, "{}: {}/{}", row.phase, row.n, row.total);
                }
            }
            out
        }
        Subject::Transcript(lines) => {
            let mut out = lines.join("\n");
            if !out.is_empty() {
                out.push('\n');
            }
            out
        }
        Subject::Text(t) => t.clone(),
    }
}
