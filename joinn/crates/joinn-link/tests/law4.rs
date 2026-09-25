//! Law 4: wires stay inside a body; hyperedges stay between bodies.

use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::FrameRegistry;
use joinn_frame::Hash;
use joinn_frame::Verdict;
use joinn_link::{BodyStore, assemble_universe, bind, check_law4, law4_refusals, parse_universe};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn load_body(rel: &str) -> Body {
    let path = corpus().join(rel);
    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", path.display()),
    };
    match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn load_cell(rel: &str) -> (Hash, Cell) {
    let path = corpus().join(rel);
    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", path.display()),
    };
    let cell = match parse_cell(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    (hash(&cell.coding), cell)
}

fn load_cells(rels: &[&str]) -> BTreeMap<Hash, Cell> {
    let mut cells = BTreeMap::new();
    for rel in rels {
        let (id, cell) = load_cell(rel);
        cells.insert(id, cell);
    }
    cells
}

#[test]
fn wrong_container_fires_both_law4_refusals() {
    let src = match fs::read_to_string(
        corpus()
            .join("phase5")
            .join("controls")
            .join("wrong_container.universe"),
    ) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let refusals = law4_refusals(&u);
    assert_eq!(refusals.len(), 2, "{refusals:?}");
    assert!(
        refusals.iter().any(|r| {
            r.reason.contains("inside") && r.reason.contains("calc") && r.reason.contains("wire")
        }),
        "{refusals:?}"
    );
    assert!(
        refusals.iter().any(|r| {
            r.reason.contains("wire")
                && r.reason.contains("hyperedge")
                && r.reason.contains("calc")
                && r.reason.contains("units")
        }),
        "{refusals:?}"
    );
    match check_law4(&u) {
        Verdict::Refused(_) => {}
        Verdict::Ok(()) => panic!("Law 4 must refuse"),
    }
}

#[test]
fn well_formed_universe_still_assembles() {
    let src = match fs::read_to_string(corpus().join("phase5").join("universe.universe")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_law4(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let calc_cells = load_cells(&[
        "phase0/sum.cell",
        "phase0/format.cell",
        "phase0/cli_input.cell",
    ]);
    let units_cells = load_cells(&["phase21/mul.cell"]);
    let mut store = BodyStore::new();
    match store.insert(
        load_body("phase2/calculator.body"),
        calc_cells,
        "phase2/calculator.body",
    ) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match store.insert(
        load_body("phase5/units.body"),
        units_cells,
        "phase5/units.body",
    ) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let bound = match bind(&u, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match assemble_universe(&u, &bound) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}
