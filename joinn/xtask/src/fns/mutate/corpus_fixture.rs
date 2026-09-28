//! Test fixture: every admitted corpus cell, and a store holding the named bodies.

use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;
use joinn_dna::{Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::fs;

pub(crate) fn corpus_fixture(bodies: &[&str]) -> (BTreeMap<Hash, Cell>, BodyStore) {
    let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
    let frames = FrameRegistry::phase1();
    let mut paths = Vec::new();
    walk_cells(&root.join("corpus"), &mut paths).unwrap_or_else(|e| panic!("{e}"));
    let mut cells = BTreeMap::new();
    for path in &paths {
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        if let Verdict::Ok(cell) = parse_cell(&text, &frames) {
            cells.entry(hash(&cell.coding)).or_insert(cell);
        }
    }
    let mut store = BodyStore::new();
    for rel in bodies {
        let text = fs::read_to_string(root.join("corpus").join(rel))
            .unwrap_or_else(|e| panic!("{rel}: {e}"));
        let body = match parse_body(&text, &frames) {
            Verdict::Ok(body) => body,
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        };
        match store.insert(body, cells.clone(), rel) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        }
    }
    (cells, store)
}
