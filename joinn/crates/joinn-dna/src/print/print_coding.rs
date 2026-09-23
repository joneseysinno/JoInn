//! Canonical coding-region text, including the trailing newline.

use std::fmt::Write as _;

use crate::model::CodingRegion;
use crate::print::print_contract::print_contract;
use crate::print::print_formula::print_formula;
use crate::print::print_witness::print_witness;

/// Canonical coding-region text, including the trailing newline.
pub fn print_coding(coding: &CodingRegion) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "codex {}", coding.codex);
    let _ = writeln!(out, "contract");
    print_contract(&mut out, coding);
    let _ = writeln!(out, "declarations");
    for d in &coding.declarations {
        let _ = writeln!(out, "{}", d.text);
    }
    let _ = writeln!(out, "founding");
    let mut founding = coding.founding.clone();
    founding.sort();
    for w in &founding {
        print_witness(&mut out, w);
    }
    let _ = writeln!(out, "frame {}", coding.frame);
    let _ = writeln!(out, "laws");
    for (name, law) in &coding.laws {
        let _ = writeln!(out, "{}: {}", name.0, print_formula(&law.formula));
    }
    match coding.lineage {
        Some(h) => {
            let _ = writeln!(out, "lineage {}", h.to_hex());
        }
        None => {
            let _ = writeln!(out, "lineage none");
        }
    }
    if !coding.turns.is_empty() {
        let _ = writeln!(out, "turn");
        let mut turns = coding.turns.clone();
        turns.sort_by_key(|t| t.out);
        for t in &turns {
            let mut froms = t.from.clone();
            froms.sort();
            let nums: Vec<String> = froms.iter().map(|p| p.to_string()).collect();
            let _ = writeln!(out, "{} from {{{}}}", t.out, nums.join(" "));
        }
    }
    out
}
