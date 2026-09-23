//! Print the contract section of a coding region.

use std::fmt::Write as _;

use crate::model::{CodingRegion, Direction, JoinPolicy};
use crate::print::print_formula::print_formula;

/// Print the contract section of a coding region.
pub(in crate::print) fn print_contract(out: &mut String, coding: &CodingRegion) {
    let c = &coding.contract;
    let _ = writeln!(out, "ensure");
    for (pos, f) in &c.ensure {
        let _ = writeln!(out, "{pos}: {}", print_formula(f));
    }
    let join = match c.join_policy {
        JoinPolicy::Refuse => "refuse",
        JoinPolicy::Latest => "latest",
        JoinPolicy::Queue => "queue",
    };
    let _ = writeln!(out, "join {join}");
    let mut ports = c.ports.clone();
    ports.sort_by_key(|p| p.position);
    for p in &ports {
        let dir = match p.direction {
            Direction::In => "in",
            Direction::Out => "out",
        };
        let req = if p.required { "required" } else { "optional" };
        let _ = writeln!(out, "port {} {dir} {} {req}", p.position, p.frame);
    }
    let _ = writeln!(out, "require");
    for (pos, f) in &c.require {
        let _ = writeln!(out, "{pos}: {}", print_formula(f));
    }
    if c.retired.is_empty() {
        let _ = writeln!(out, "retired");
    } else {
        let mut retired = c.retired.clone();
        retired.sort();
        let nums: Vec<String> = retired.iter().map(|n| n.to_string()).collect();
        let _ = writeln!(out, "retired {}", nums.join(" "));
    }
}
