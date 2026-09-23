//! Canonical allele payload text.

use std::fmt::Write as _;

use crate::model::{Allele, AlleleBody};
use crate::print::print_witness::print_witness;

/// Canonical allele payload text.
pub fn print_allele(allele: &Allele) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "frame {}", allele.frame);
    match &allele.body {
        AlleleBody::Native(id) => {
            let _ = writeln!(out, "native {}", id.0);
        }
        AlleleBody::Dna(h) => {
            let _ = writeln!(out, "dna {}", h.to_hex());
        }
    }
    let _ = writeln!(out, "witnesses");
    let mut ws = allele.witnesses.clone();
    ws.sort();
    for w in &ws {
        print_witness(&mut out, w);
    }
    out
}
