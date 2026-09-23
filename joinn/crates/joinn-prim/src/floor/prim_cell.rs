//! A synthetic cell for a `prim:` genome instance.

use joinn_dna::{Allele, AlleleBody, Cell, CodingRegion, Contract, JoinPolicy, NativeId};
use joinn_frame::FrameRef;
use std::collections::BTreeMap;

use super::prim_ports::prim_ports;

/// A synthetic cell for a `prim:` genome instance. Not gated; not hashed as a cell.
pub fn prim_cell(name: &str) -> Option<Cell> {
    let ports = prim_ports(name)?;
    Some(Cell {
        coding: CodingRegion {
            codex: 1,
            frame: FrameRef::int(),
            contract: Contract {
                ports,
                retired: Vec::new(),
                join_policy: JoinPolicy::Latest,
                require: BTreeMap::new(),
                ensure: BTreeMap::new(),
            },
            laws: BTreeMap::new(),
            founding: Vec::new(),
            declarations: Vec::new(),
            lineage: None,
            turns: Vec::new(),
        },
        regulatory: Default::default(),
        alleles: vec![Allele {
            frame: FrameRef::int(),
            body: AlleleBody::Native(NativeId(format!("prim:{name}"))),
            witnesses: Vec::new(),
        }],
    })
}
