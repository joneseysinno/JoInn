//! Calculator under the headless host.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Term, Verdict};
use joinn_host::{Address, Role, print_description};
use joinn_test_host::{RawEvent, run};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn load_calculator() -> (
    joinn_dna::Body,
    BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
) {
    let corpus = corpus();
    let frames = FrameRegistry::phase1();
    let src = match fs::read_to_string(corpus.join("phase2").join("calculator.body")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let body = match parse_body(&src, &frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut cells = BTreeMap::new();
    for name in ["sum", "format", "cli_input"] {
        let path = corpus.join("phase0").join(format!("{name}.cell"));
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
    (body, cells)
}

fn event(instance: &str, line: &str) -> RawEvent {
    RawEvent {
        address: Address {
            instance: instance.into(),
            port: 0,
        },
        term: Term::text(line),
    }
}

#[test]
fn calculator_end_to_end_captures_fires_and_refusal() {
    let (body, cells) = load_calculator();
    let natives = joinn_prim::sealed_natives();

    let refused = match run(
        body.clone(),
        cells.clone(),
        natives.clone(),
        vec![event("cli_a", "two")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        refused
            .descriptions
            .iter()
            .any(|d| d.role == Role::Refusal && d.label.contains("two")),
        "expected a refusal description for two, got {:?}",
        refused
            .descriptions
            .iter()
            .map(|d| (&d.role, &d.label))
            .collect::<Vec<_>>()
    );

    let ok = match run(
        body,
        cells,
        natives,
        vec![event("cli_a", "2"), event("cli_b", "3")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let fires: Vec<_> = ok
        .descriptions
        .iter()
        .filter(|d| d.role == Role::Cell)
        .collect();
    assert!(
        !fires.is_empty(),
        "expected one description per fire, got none"
    );
    let sum = fires.iter().find(|d| d.instance == "sum");
    let Some(sum) = sum else {
        panic!(
            "expected a sum fire, got {:?}",
            fires.iter().map(|d| &d.instance).collect::<Vec<_>>()
        );
    };
    let vals: Vec<_> = sum
        .ports
        .iter()
        .filter_map(|p| p.value.as_deref())
        .collect();
    assert_eq!(vals, ["2", "3", "5"]);
    let ports: usize = ok.intent_set.values().map(|set| set.len()).sum();
    assert_eq!(ports, 2);
    let got = print_description(sum);
    let want = match fs::read_to_string(corpus().join("descriptions").join("calculator.desc")) {
        Ok(s) => s.replace("\r\n", "\n"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(got, want);
}
