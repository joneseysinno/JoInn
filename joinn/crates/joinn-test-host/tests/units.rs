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
    for name in ["format", "cli_input"] {
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
    let natives = joinn_prim::sealed_natives();
    let refused = match run(
        body.clone(),
        cells.clone(),
        natives.clone(),
        vec![RawEvent {
            address: Address {
                instance: "scale".into(),
                port: 0,
            },
            term: Term::text("two"),
        }],
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
        vec![RawEvent {
            address: Address {
                instance: "scale".into(),
                port: 0,
            },
            term: Term::text("7"),
        }],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        ok.descriptions.iter().any(|d| {
            d.instance == "scale"
                && d.ports
                    .iter()
                    .any(|p| p.position == 1 && p.value.as_deref() == Some("7"))
        }),
        "expected scale out-port 7, got {:?}",
        ok.descriptions
    );
}
