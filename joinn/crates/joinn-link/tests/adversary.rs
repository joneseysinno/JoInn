//! Law 4 adversary: a third body under a wire and a hyperedge.
//! The Phase 5 bus still satisfies Law 4, but typed links refuse:
//! `units` is echo (cli_input), so scale@1 is Out and may not be marked head.

use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{BodyStore, bind, check_law4, check_link_types, parse_universe};
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

fn cells_with_mul() -> BTreeMap<Hash, Cell> {
    let mut cells = BTreeMap::new();
    for rel in [
        "phase0/sum.cell",
        "phase0/format.cell",
        "phase0/cli_input.cell",
        "phase21/mul.cell",
    ] {
        let (id, cell) = load_cell(rel);
        cells.insert(id, cell);
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
    let cells = cells_with_mul();
    let mut store = BodyStore::new();
    for rel in [
        "phase2/calculator.body",
        "phase5/controls/echo.body",
        "phase5/bus.body",
    ] {
        let body = load_body(rel);
        match store.insert(body, cells.clone(), rel) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        }
    }
    let bound = match bind(&u, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_link_types(&u, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("units.scale@1"), "{}", r.reason);
            assert!(r.reason.contains("head"), "{}", r.reason);
            assert!(r.reason.contains("Out"), "{}", r.reason);
            assert!(r.reason.contains("acceptance is In"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("out-port marked head must refuse"),
    }
}
