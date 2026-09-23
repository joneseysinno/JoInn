//! Shared test fixtures for the live engine.
// allow(modules): shared test helpers — not production operations

use joinn_dna::{Body, Cell, cli_input_cell, format_cell, hash, parse_body, sum_cell};
use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, TextFrame, Value, Verdict};
use joinn_gate::NativeRegistry;
use std::collections::BTreeMap;

pub(crate) fn int(n: i64) -> Value {
    match IntFrame::new().canonicalize(Term::int(n)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => panic!("int"),
    }
}

pub(crate) fn text(s: &str) -> Value {
    match TextFrame::new().canonicalize(Term::text(s)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(_) => panic!("text"),
    }
}

pub(crate) fn sum_cli_body() -> (Body, BTreeMap<Hash, Cell>) {
    let format = format_cell();
    let fh = hash(&format.coding);
    let cli = cli_input_cell(fh);
    let sum = sum_cell();
    let sh = hash(&sum.coding);
    let ch = hash(&cli.coding);
    let src = format!(
        "body {{ codex 1 genome {{ cell:{} as sum cell:{} as cli_a, cli_b }} grants {{ cli cli_a cli_b }} wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }} budget {{ steps 100000 }} lineage none }}\n",
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
    (body, cells)
}

pub(crate) fn natives() -> NativeRegistry {
    joinn_prim::engine_natives()
}

pub(crate) fn tagged_sum(tag: i64) -> Cell {
    let mut c = sum_cell();
    if let Some(w) = c.coding.founding.first_mut() {
        w.outputs.insert(2, int(tag));
    }
    c
}

pub(crate) fn print_body_eq(body: &Body) -> String {
    joinn_dna::print_body(&body.coding)
}
