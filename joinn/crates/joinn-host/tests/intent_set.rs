//! The calculator's intent set is the body's, not the host's.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Term, Verdict};
use joinn_host::{Address, Intent, check_intent, intent_set};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn load_body(rel: &str) -> joinn_dna::Body {
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

fn cells_named(names: &[&str]) -> BTreeMap<Hash, joinn_dna::Cell> {
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

fn calculator_body() -> joinn_dna::Body {
    load_body("phase2/calculator.body")
}

fn calculator_cells() -> BTreeMap<Hash, joinn_dna::Cell> {
    cells_named(&["sum", "format", "cli_input"])
}

fn addr(instance: &str, port: u32) -> Address {
    Address {
        instance: instance.into(),
        port,
    }
}

fn intent_at(instance: &str, port: u32) -> Intent {
    Intent {
        address: addr(instance, port),
        term: Term::text("2"),
    }
}

#[test]
fn intent_set_is_the_membrane_in_ports() {
    let body = calculator_body();
    let got = intent_set(&body, &calculator_cells());
    let want = BTreeSet::from([addr("cli_a", 0), addr("cli_b", 0)]);
    assert_eq!(got, want);
}

#[test]
fn two_in_ports_are_both_in_the_set() {
    let body = load_body("phase5/two_in_ports.body");
    let got = intent_set(&body, &cells_named(&["sum"]));
    let want = BTreeSet::from([addr("pair", 0), addr("pair", 1)]);
    assert_eq!(got, want);
}

#[test]
fn intent_at_sum_is_refused_naming_the_address() {
    let body = calculator_body();
    match check_intent(&body, &calculator_cells(), &intent_at("sum", 0)) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("sum@0"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("intent at sum@0 must refuse"),
    }
}

#[test]
fn declared_intents_are_accepted() {
    let body = calculator_body();
    for instance in ["cli_a", "cli_b"] {
        match check_intent(&body, &calculator_cells(), &intent_at(instance, 0)) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{instance}@0 must be accepted: {}", r.reason),
        }
    }
}
