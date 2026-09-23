//! Environment signals: declared by the host, read by the body.

use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_host::{Signals, check_signals, signals_from_environment};
use joinn_live::BodyState;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

fn load_body(rel: &str) -> joinn_dna::Body {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
        .join(rel);
    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", path.display()),
    };
    match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn run_under(body: joinn_dna::Body, host: &Signals) -> Verdict<()> {
    match check_signals(&body, host) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let mut state = match BodyState::new(body, Default::default(), joinn_prim::engine_natives(), 1)
    {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    match state.run() {
        Verdict::Ok(_) => Verdict::Ok(()),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[test]
fn calculator_reads_nothing_and_runs_under_none() {
    let body = load_body("phase2/calculator.body");
    let cells_ok = check_signals(&body, &Signals::new(BTreeSet::new()));
    match cells_ok {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    assert!(body.coding.reads.is_empty());
    // Load cells the calculator names.
    let format = joinn_dna::format_cell();
    let fh = joinn_dna::hash(&format.coding);
    let cli = joinn_dna::cli_input_cell(fh);
    let sum = joinn_dna::sum_cell();
    let mut cells = std::collections::BTreeMap::new();
    cells.insert(fh, format);
    cells.insert(joinn_dna::hash(&cli.coding), cli);
    cells.insert(joinn_dna::hash(&sum.coding), sum);
    let mut state = match BodyState::new(body, cells, joinn_prim::engine_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match state.run() {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn columns_reader_runs_when_columns_is_declared() {
    let body = load_body("phase3/columns_reader.body");
    let host = Signals::new(BTreeSet::from(["columns".into()]));
    match run_under(body, &host) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn columns_reader_is_refused_naming_columns_under_none() {
    let body = load_body("phase3/columns_reader.body");
    match run_under(body, &Signals::new(BTreeSet::new())) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("columns"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("undeclared columns must refuse"),
    }
}

#[test]
fn environment_body_emits_columns() {
    let body = load_body("phase3/environment.body");
    let signals = signals_from_environment(&body);
    assert!(signals.emits("columns"));
    let reader = load_body("phase3/columns_reader.body");
    match check_signals(&reader, &signals) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn a_body_naming_rows_does_not_emit_columns() {
    let src = "body { codex 1 genome { prim:eq as rows } grants { } wires { } budget { steps 1 } lineage none }\n";
    let body = match parse_body(src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let signals = signals_from_environment(&body);
    assert!(!signals.emits("columns"));
    let reader = load_body("phase3/columns_reader.body");
    match check_signals(&reader, &signals) {
        Verdict::Refused(r) => assert!(r.reason.contains("columns"), "{}", r.reason),
        Verdict::Ok(()) => panic!("rows must not satisfy a reader of columns"),
    }
}
