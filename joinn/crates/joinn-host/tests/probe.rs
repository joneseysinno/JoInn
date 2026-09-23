//! probe reads and never writes.

use joinn_dna::{cli_input_cell, format_cell, hash, parse_body, sum_cell};
use joinn_frame::{Frame, FrameRegistry, Term, TextFrame, Value, Verdict};
use joinn_host::{Address, probe};
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

#[test]
fn probe_does_not_draw_budget() {
    let state = calculator_after_two_plus_three();
    let before = state.steps();
    let addr = Address {
        instance: "sum".into(),
        port: 0,
    };
    match probe(&state, addr) {
        Verdict::Ok(d) => {
            assert_eq!(d.instance, "sum");
            assert_eq!(d.ports.len(), 1);
            assert_eq!(d.ports[0].position, 0);
        }
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    assert_eq!(state.steps(), before);
}

#[test]
fn probe_refuses_a_port_the_instance_does_not_declare() {
    let state = calculator_after_two_plus_three();
    let addr = Address {
        instance: "sum".into(),
        port: 9,
    };
    match probe(&state, addr) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("sum@9"), "{}", r.reason);
            assert!(r.reason.contains("acceptance"), "{}", r.reason);
        }
        Verdict::Ok(_) => panic!("port 9 is not on sum"),
    }
}
