//! Units body under the headless host.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Term, Verdict};
use joinn_host::{Address, Role};
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

fn event(port: u32, term: Term) -> RawEvent {
    RawEvent {
        address: Address {
            instance: "scale".into(),
            port,
        },
        term,
    }
}

#[test]
fn units_runs_standalone_under_test_host() {
    let corpus = corpus();
    let frames = FrameRegistry::phase1();
    let src = match fs::read_to_string(corpus.join("phase5").join("units.body")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let body = match parse_body(&src, &frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut cells = BTreeMap::new();
    let mul_path = corpus.join("phase21").join("mul.cell");
    let mul_src = match fs::read_to_string(&mul_path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", mul_path.display()),
    };
    let mul = match parse_cell(&mul_src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    cells.insert(hash(&mul.coding), mul);
    let natives = joinn_prim::sealed_natives();
    let refused = match run(
        body.clone(),
        cells.clone(),
        natives.clone(),
        vec![event(0, Term::text("two"))],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        refused
            .descriptions
            .iter()
            .any(|d| d.role == Role::Refusal && d.label.contains("two")),
        "expected a refusal for two"
    );
    let ok = match run(
        body,
        cells,
        natives,
        vec![event(0, Term::int(7)), event(1, Term::int(12))],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        ok.descriptions.iter().any(|d| {
            d.instance == "scale"
                && d.ports
                    .iter()
                    .any(|p| p.position == 2 && p.value.as_deref() == Some("84"))
        }),
        "expected scale out-port 84, got {:?}",
        ok.descriptions
    );
}
