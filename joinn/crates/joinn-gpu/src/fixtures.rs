//! Test fixture: the calculator body and its cells, parsed from the corpus.

use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;

/// `phase2/calculator.body` with `sum`, `format` and `cli_input` from `phase0`.
pub(crate) fn calculator() -> (Body, BTreeMap<Hash, Cell>) {
    let frames = FrameRegistry::phase1();
    let body = match parse_body(
        include_str!("../../../corpus/phase2/calculator.body"),
        &frames,
    ) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut cells = BTreeMap::new();
    for src in [
        include_str!("../../../corpus/phase0/sum.cell"),
        include_str!("../../../corpus/phase0/format.cell"),
        include_str!("../../../corpus/phase0/cli_input.cell"),
    ] {
        match parse_cell(src, &frames) {
            Verdict::Ok(c) => {
                cells.insert(hash(&c.coding), c);
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
    (body, cells)
}
