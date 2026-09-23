//! `check_body`.

#![allow(clippy::result_large_err)]

use crate::model::Cell;
use joinn_frame::{CheckId, Hash, Refusal, Subject, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use crate::body::{Body, GenomeTarget};

/// Structural body check. Unknown cell hashes and wires to missing ports refuse.
pub fn check_body(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<()> {
    let mut names: BTreeSet<String> = BTreeSet::new();
    for g in &body.coding.genome {
        if let GenomeTarget::Cell(cell) = g.target {
            if !cells.contains_key(&cell) {
                return Verdict::Refused(Refusal {
                    check: CheckId::Contract,
                    subject: Subject::Cell(cell),
                    reason: format!("unknown cell hash {}", cell.to_hex()),
                    counterexample: None,
                    seed: 0,
                });
            }
        }
        for inst in &g.instances {
            if !names.insert(inst.clone()) {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Contract,
                    format!("duplicate instance name {inst}"),
                ));
            }
        }
    }
    let inst_cell: BTreeMap<&str, Option<Hash>> = body
        .coding
        .genome
        .iter()
        .flat_map(|g| {
            let h = match &g.target {
                GenomeTarget::Cell(h) => Some(*h),
                GenomeTarget::Prim(_) => None,
            };
            g.instances.iter().map(move |i| (i.as_str(), h))
        })
        .collect();
    for w in &body.coding.wires {
        let Some(src_h) = inst_cell.get(w.src_instance.as_str()) else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!("wire source instance {} does not exist", w.src_instance),
            ));
        };
        let Some(dst_h) = inst_cell.get(w.dst_instance.as_str()) else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Contract,
                format!(
                    "wire destination instance {} does not exist",
                    w.dst_instance
                ),
            ));
        };
        match (src_h, dst_h) {
            (Some(src), Some(dst)) => {
                let src_cell = &cells[src];
                let dst_cell = &cells[dst];
                let src_ok = src_cell
                    .coding
                    .contract
                    .ports
                    .iter()
                    .any(|p| p.position == w.src_port);
                if !src_ok {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        format!(
                            "wire to nonexistent out-port {}@{}",
                            w.src_instance, w.src_port
                        ),
                    ));
                }
                let dst_ok = dst_cell
                    .coding
                    .contract
                    .ports
                    .iter()
                    .any(|p| p.position == w.dst_port);
                if !dst_ok {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        format!(
                            "wire to nonexistent in-port {}@{}",
                            w.dst_instance, w.dst_port
                        ),
                    ));
                }
            }
            _ => {
                // A prim: end is resolved by the live engine against the floor.
            }
        }
    }
    Verdict::Ok(())
}
