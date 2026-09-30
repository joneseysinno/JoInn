//! The contact calculator under the headless host describes itself exactly as
//! the wired one does.

use joinn_dna::{hash, parse_cell, parse_contact};
use joinn_frame::{FrameRegistry, Term, Verdict};
use joinn_host::{Address, Role, print_description};
use joinn_test_host::{RawEvent, run_contact};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn read(path: PathBuf) -> String {
    match fs::read_to_string(&path) {
        Ok(s) => s.replace("\r\n", "\n"),
        Err(e) => panic!("{}: {e}", path.display()),
    }
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
fn the_contact_calculator_describes_itself_as_the_goldens_say() {
    let frames = FrameRegistry::phase1();
    let contact = match parse_contact(
        &read(corpus().join("phase7").join("calculator.contact")),
        &frames,
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut cells = BTreeMap::new();
    for name in ["sum", "cli_input"] {
        match parse_cell(
            &read(corpus().join("phase0").join(format!("{name}.cell"))),
            &frames,
        ) {
            Verdict::Ok(c) => {
                cells.insert(hash(&c.coding), c);
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
    let script = vec![
        event("cli_a", "two"),
        event("cli_a", "2"),
        event("cli_b", "3"),
    ];
    let cap = match run_contact(&contact, cells, joinn_prim::sealed_natives(), script) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let Some(refusal) = cap.descriptions.iter().find(|d| d.role == Role::Refusal) else {
        panic!("no refusal after event 1");
    };
    assert_eq!(
        print_description(refusal),
        read(
            corpus()
                .join("descriptions")
                .join("calculator_refusal.desc")
        )
    );
    let Some(sum) = cap.descriptions.iter().rev().find(|d| d.instance == "sum") else {
        panic!("sum never fired");
    };
    assert_eq!(
        print_description(sum),
        read(corpus().join("descriptions").join("calculator.desc"))
    );
    let ports: usize = cap.intent_set.values().map(|set| set.len()).sum();
    assert_eq!(ports, 2);
}

#[test]
fn a_refused_contact_does_not_run() {
    let frames = FrameRegistry::phase1();
    let contact = match parse_contact(
        &read(corpus().join("phase7").join("calculator.contact")),
        &frames,
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let Verdict::Refused(r) = run_contact(
        &contact,
        BTreeMap::new(),
        joinn_prim::sealed_natives(),
        vec![event("cli_a", "2")],
    ) else {
        panic!("a contact whose cells are missing must be refused");
    };
    assert_eq!(
        r.reason,
        "unknown cell hash c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e"
    );
}
