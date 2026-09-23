//! `print_body`.

#![allow(clippy::result_large_err)]

use joinn_frame::Hash;
use std::fmt::Write as _;

use crate::body::{BodyCoding, GenomeTarget};

/// Canonical body text, including the trailing newline.
pub fn print_body(coding: &BodyCoding) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "budget");
    let _ = writeln!(out, "steps {}", coding.budget_steps);
    let _ = writeln!(out, "codex {}", coding.codex);
    let _ = writeln!(out, "genome");
    let mut genome = coding.genome.clone();
    for g in &mut genome {
        g.instances.sort();
    }
    genome.sort_by(|a, b| match (&a.target, &b.target) {
        (GenomeTarget::Cell(x), GenomeTarget::Cell(y)) => {
            x.cmp(y).then(a.instances.cmp(&b.instances))
        }
        (GenomeTarget::Cell(_), GenomeTarget::Prim(_)) => std::cmp::Ordering::Less,
        (GenomeTarget::Prim(_), GenomeTarget::Cell(_)) => std::cmp::Ordering::Greater,
        (GenomeTarget::Prim(x), GenomeTarget::Prim(y)) => {
            x.cmp(y).then(a.instances.cmp(&b.instances))
        }
    });
    // Flatten to (cell hash, instance) then group. Prim entries are printed after.
    let mut pairs: Vec<(Hash, String)> = Vec::new();
    let mut prims: Vec<(String, String)> = Vec::new();
    for g in &coding.genome {
        match &g.target {
            GenomeTarget::Cell(h) => {
                for inst in &g.instances {
                    pairs.push((*h, inst.clone()));
                }
            }
            GenomeTarget::Prim(name) => {
                for inst in &g.instances {
                    prims.push((name.clone(), inst.clone()));
                }
            }
        }
    }
    pairs.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut grouped: Vec<(Hash, Vec<String>)> = Vec::new();
    for (h, inst) in pairs {
        match grouped.last_mut() {
            Some((gh, list)) if *gh == h => list.push(inst),
            _ => grouped.push((h, vec![inst])),
        }
    }
    for (h, insts) in grouped {
        let names = insts.join(", ");
        let _ = writeln!(out, "cell:{} as {names}", h.to_hex());
    }
    prims.sort();
    let mut grouped_p: Vec<(String, Vec<String>)> = Vec::new();
    for (name, inst) in prims {
        match grouped_p.last_mut() {
            Some((pn, list)) if *pn == name => list.push(inst),
            _ => grouped_p.push((name, vec![inst])),
        }
    }
    for (name, insts) in grouped_p {
        let names = insts.join(", ");
        let _ = writeln!(out, "prim:{name} as {names}");
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
    let _ = writeln!(out, "wires");
    let mut wires = coding.wires.clone();
    wires.sort_by(|a, b| {
        a.src_instance
            .cmp(&b.src_instance)
            .then(a.src_port.cmp(&b.src_port))
            .then(a.dst_instance.cmp(&b.dst_instance))
            .then(a.dst_port.cmp(&b.dst_port))
    });
    for w in wires {
        let _ = writeln!(
            out,
            "{}@{} -> {}@{}",
            w.src_instance, w.src_port, w.dst_instance, w.dst_port
        );
    }
    out
}
