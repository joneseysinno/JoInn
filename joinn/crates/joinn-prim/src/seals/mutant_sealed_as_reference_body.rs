//! Purpose-built mutant 14: a genome that names the sealed native.

use joinn_dna::Body;
use std::collections::{BTreeMap, BTreeSet};

/// Purpose-built mutant 14: a genome that names the sealed native.
pub fn mutant_sealed_as_reference_body() -> Body {
    Body {
        coding: joinn_dna::BodyCoding {
            codex: 1,
            genome: vec![joinn_dna::GenomeEntry {
                target: joinn_dna::GenomeTarget::Prim("add@ℤ".into()),
                instances: vec!["x".into()],
            }],
            grants: BTreeMap::new(),
            reads: BTreeSet::new(),
            wires: Vec::new(),
            budget_steps: 1,
            lineage: None,
        },
        regulatory: Default::default(),
    }
}
