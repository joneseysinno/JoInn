//! Law 4 adversary: a third body expressible with a wire and a hyperedge.

use joinn_dna::{hash, parse_body, parse_cell, Body, Cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{assemble_universe, check_law4, parse_universe};
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

#[test]
fn three_body_bus_is_expressible_under_law4() {
    let src = match fs::read_to_string(corpus().join("phase5").join("adversary.universe")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_law4(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("Law 4 must hold for the bus: {}", r.reason),
    }
    let cells = load_cells(&["sum", "format", "cli_input"]);
    let mut bodies = BTreeMap::new();
    bodies.insert(
        "calc".into(),
        (load_body("phase2/calculator.body"), cells.clone()),
    );
    bodies.insert(
        "units".into(),
        (load_body("phase5/units.body"), cells.clone()),
    );
    bodies.insert("bus".into(), (load_body("phase5/bus.body"), cells));
    match assemble_universe(&u, &bodies) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}
