//! describe on the calculator's sum instance after 2 + 3.

use joinn_dna::{cli_input_cell, format_cell, hash, parse_body, sum_cell};
use joinn_frame::{
    Frame, FrameRegistry, TAG_DESCRIPTION, Term, TextFrame, Value, Verdict, keyed_hash,
};
use joinn_host::{describe, hash_description, print_description};
use joinn_live::BodyState;
use std::collections::BTreeMap;

fn text(s: &str) -> Value {
    match TextFrame::new().canonicalize(Term::text(s)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn calculator_after_two_plus_three() -> BodyState {
    let format = format_cell();
    let fh = hash(&format.coding);
    let cli = cli_input_cell(fh);
    let sum = sum_cell();
    let sh = hash(&sum.coding);
    let ch = hash(&cli.coding);
    let src = format!(
        "body {{ codex 1 genome {{ cell:{} as sum cell:{} as cli_a, cli_b }} grants {{ cli cli_a cli_b }} wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }} budget {{ steps 100000 }} lineage none }}\n---\nregulatory {{ names {{ sum \"Sum\" }} labels {{ sum \"Sum\" }} }}\n",
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
    match state.inject("cli_a", 0, text("2"), 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match state.inject("cli_b", 0, text("3"), 1) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match state.run() {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    state
}

const SUM_AFTER_TWO_PLUS_THREE: &str = "\
description {
  cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
  instance sum
  label \"Sum\"
  role cell
  port 0 in ℤ 1 \"a\" label \"first addend\" role input value 2
  port 1 in ℤ 1 \"b\" label \"second addend\" role input value 3
  port 2 out ℤ 1 \"result\" label \"sum\" role output value 5
}
";

// Hand-computed: BLAKE3-256(len_le(tag) ‖ joinn.description.v1 ‖ len_le(bytes) ‖ canonical_text)
// of SUM_AFTER_TWO_PLUS_THREE. Filled after independent keyed_hash of those bytes.
const SUM_DESCRIPTION_HASH: &str =
    "5252d61ba2ac9c1c17a1f1af1453ffc9f36cfe248df27547035213603a20d3a5";

#[test]
fn describe_sum_matches_section_3_3() {
    let state = calculator_after_two_plus_three();
    let d = match describe(&state, "sum") {
        Verdict::Ok(d) => d,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let text = print_description(&d);
    assert_eq!(text, SUM_AFTER_TWO_PLUS_THREE);
    let h = hash_description(&d);
    assert_eq!(h, keyed_hash(TAG_DESCRIPTION, text.as_bytes()));
    assert_eq!(
        h.to_hex(),
        SUM_DESCRIPTION_HASH,
        "hand-computed hash mismatch; machine produced {}",
        h.to_hex()
    );
}

#[test]
fn unknown_instance_is_refused_by_name() {
    let state = calculator_after_two_plus_three();
    match describe(&state, "missing") {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("missing"), "{}", r.reason);
        }
        Verdict::Ok(_) => panic!("unknown instance must refuse"),
    }
}
