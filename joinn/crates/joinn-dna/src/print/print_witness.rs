//! Print one witness block.

use std::fmt::Write as _;

use crate::model::Witness;

/// Print one witness block.
pub(in crate::print) fn print_witness(out: &mut String, w: &Witness) {
    let _ = writeln!(out, "witness");
    for (pos, v) in &w.inputs {
        let _ = writeln!(out, "in {pos}: {}", v.print_literal());
    }
    for (pos, v) in &w.outputs {
        let _ = writeln!(out, "out {pos}: {}", v.print_literal());
    }
}
