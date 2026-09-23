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

fn load_cells(names: &[&str]) -> BTreeMap<Hash, Cell> {
    let frames = FrameRegistry::phase1();
    let mut cells = BTreeMap::new();
    for name in names {
        let path = corpus().join("phase0").join(format!("{name}.cell"));
        let src = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => panic!("{}: {e}", path.display()),
        };
        let cell = match parse_cell(&src, &frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        cells.insert(hash(&cell.coding), cell);
    }
    cells
}

fn loaded_bodies() -> BTreeMap<String, (Body, BTreeMap<Hash, Cell>)> {
    let calc_cells = load_cells(&["sum", "format", "cli_input"]);
    let units_cells = load_cells(&["format", "cli_input"]);
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
