//! Touch-only: assemble refuses an interior port by name.

use joinn_dna::{hash, parse_body, parse_cell, Body, Cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{assemble_universe, parse_universe};
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

fn loaded_bodies() -> BTreeMap<String, (Body, BTreeMap<Hash, Cell>)> {
    let calc_cells = load_cells(&[
        "phase0/sum.cell",
        "phase0/format.cell",
        "phase0/cli_input.cell",
    ]);
    let units_cells = load_cells(&["phase21/mul.cell"]);
    let mut bodies = BTreeMap::new();
    bodies.insert(
        "calc".into(),
        (load_body("phase2/calculator.body"), calc_cells),
    );
    bodies.insert("units".into(), (load_body("phase5/units.body"), units_cells));
    bodies
}

#[test]
fn universe_universe_assembles() {
    let src = match fs::read_to_string(corpus().join("phase5").join("universe.universe")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match assemble_universe(&u, &loaded_bodies()) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn transits_refuses_naming_calc_sum_0_and_the_wire() {
    let src = match fs::read_to_string(
        corpus()
            .join("phase5")
            .join("controls")
            .join("transits.universe"),
    ) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match assemble_universe(&u, &loaded_bodies()) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("calc.sum@0"), "{}", r.reason);
            assert!(r.reason.contains("interior"), "{}", r.reason);
            assert!(
                r.reason.contains("cli_a@1 -> sum@0"),
                "{}",
                r.reason
            );
        }
        Verdict::Ok(()) => panic!("transiting sum@0 must refuse"),
    }
}
