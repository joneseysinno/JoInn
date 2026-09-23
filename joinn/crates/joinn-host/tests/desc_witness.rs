//! Description witnesses: goldens, both hosts, regulatory vs coding.

use joinn_dna::{cli_input_cell, format_cell, hash, parse_body, sum_cell};
use joinn_frame::{Frame, FrameRegistry, Term, TextFrame, Value, Verdict};
use joinn_host::{describe, describe_refusal, print_description};
use joinn_live::BodyState;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn text(s: &str) -> Value {
    match TextFrame::new().canonicalize(Term::text(s)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn desc_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
        .join("descriptions")
        .join(name)
}

fn calculator_state(a: &str, b: Option<&str>) -> (BodyState, String) {
    calculator_labeled(a, b, "Sum")
}

fn calculator_labeled(a: &str, b: Option<&str>, sum_label: &str) -> (BodyState, String) {
    let format = format_cell();
    let fh = hash(&format.coding);
    let cli = cli_input_cell(fh);
    let sum = sum_cell();
    let sh = hash(&sum.coding);
    let ch = hash(&cli.coding);
    let src = format!(
        "body {{ codex 1 genome {{ cell:{} as sum cell:{} as cli_a, cli_b }} grants {{ cli cli_a cli_b }} wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }} budget {{ steps 100000 }} lineage none }}\n---\nregulatory {{ names {{ sum \"Sum\" }} labels {{ sum \"{sum_label}\" }} }}\n",
        sh.to_hex(),
        ch.to_hex()
    );
    let body = match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut cells = BTreeMap::new();
    cells.insert(fh, format);
    cells.insert(ch, cli);
    cells.insert(sh, sum);
    let mut state = match BodyState::new(body, cells, joinn_prim::engine_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match state.inject("cli_a", 0, text(a), 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    if let Some(b) = b {
        match state.inject("cli_b", 0, text(b), 1) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        match state.run() {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        (state, String::new())
    } else {
        let reason = match state.run() {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(_) => panic!("expected refusal"),
        };
        (state, reason)
    }
}

#[test]
fn calculator_desc_matches_golden() {
    let (state, _) = calculator_state("2", Some("3"));
    let d = match describe(&state, "sum") {
        Verdict::Ok(d) => d,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let got = print_description(&d);
    let want = match fs::read_to_string(desc_path("calculator.desc")) {
        Ok(s) => s.replace("\r\n", "\n"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(got, want);
}

#[test]
fn calculator_refusal_desc_matches_golden() {
    let (state, reason) = calculator_state("two", None);
    let d = describe_refusal(&state, "cli_a", &reason);
    let got = print_description(&d);
    let path = desc_path("calculator_refusal.desc");
    if !path.exists() {
        panic!("refusal description (write this golden):\n{got}");
    }
    let want = match fs::read_to_string(&path) {
        Ok(s) => s.replace("\r\n", "\n"),
        Err(e) => panic!("{e}"),
    };
    assert_eq!(got, want);
}

#[test]
fn regulatory_label_moves_description_not_cell_hash() {
    let cell = sum_cell();
    let before_cell = hash(&cell.coding);
    let (before_state, _) = calculator_labeled("2", Some("3"), "Sum");
    let (after_state, _) = calculator_labeled("2", Some("3"), "the total");
    let before = match describe(&before_state, "sum") {
        Verdict::Ok(d) => print_description(&d),
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let after = match describe(&after_state, "sum") {
        Verdict::Ok(d) => print_description(&d),
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(hash(&sum_cell().coding), before_cell);
    match first_desc_diff(&before, &after) {
        Some(field) => assert_eq!(field, "label", "{before}\n---\n{after}"),
        None => panic!("label edit did not move the description"),
    }
}

fn first_desc_diff(want: &str, got: &str) -> Option<String> {
    for (a, b) in want.lines().zip(got.lines()) {
        if a == b {
            continue;
        }
        let ta: Vec<&str> = a.split_whitespace().collect();
        let tb: Vec<&str> = b.split_whitespace().collect();
        let n = ta.len().min(tb.len());
        for i in 0..n {
            if ta[i] != tb[i] {
                if i > 0 {
                    return Some(ta[i - 1].to_string());
                }
                return Some(ta[i].to_string());
            }
        }
        return Some("line".into());
    }
    None
}
