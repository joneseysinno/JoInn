//! Universe parse, print, hash, CSR, and undeclared-alias refusal.

use joinn_frame::{TAG_UNIVERSE, Verdict, keyed_hash};
use joinn_link::{
    Order, coding_from_csr, csr_from_universe, hash_universe, parse_universe, print_universe,
};
use std::fs;
use std::path::PathBuf;

fn load(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
        .join(rel);
    match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", path.display()),
    }
}

fn round_trip(src: &str) -> String {
    let u = match parse_universe(src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let once = print_universe(&u.coding);
    let again = match parse_universe(&once) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let twice = print_universe(&again.coding);
    assert_eq!(once, twice);
    once
}

#[test]
fn three_universes_round_trip() {
    let a = round_trip(&load("phase5/universe.universe"));
    let b = round_trip(&load("phase5/alone.universe"));
    let c = round_trip(&load("phase5/ordered.universe"));
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert!(c.contains("units.scale@0"));
    assert!(c.contains("calc.sum@2"));
    let idx_units = match c.find("units.scale@0") {
        Some(i) => i,
        None => panic!("units first in ordered path"),
    };
    let idx_calc = match c.find("calc.sum@2") {
        Some(i) => i,
        None => panic!("calc second in ordered path"),
    };
    assert!(idx_calc < idx_units);
}

#[test]
fn undeclared_body_alias_is_refused_by_name() {
    let src = load("phase5/controls/undeclared.universe");
    match parse_universe(&src) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("ghost"), "{}", r.reason);
            assert!(r.reason.contains("acceptance"), "{}", r.reason);
        }
        Verdict::Ok(_) => panic!("undeclared alias must refuse"),
    }
}

#[test]
fn hand_computed_universe_hash_matches() {
    let src = load("phase5/universe.universe");
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let text = print_universe(&u.coding);
    let h = hash_universe(&u.coding);
    assert_eq!(h, keyed_hash(TAG_UNIVERSE, text.as_bytes()));
    // Hand-computed: BLAKE3-256(len_le(tag) ‖ joinn.universe.v1 ‖ len_le(bytes) ‖ canonical_text)
    // of print_universe(universe.universe) after P51-12. Recorded in phase-5.1-hashes.md.
    assert_eq!(
        h.to_hex(),
        "2ccbb067db0b150ca607d1e7469fd28ea46892142d6b7729f1075cb899c9d1ad"
    );
}

#[test]
fn csr_round_trips_to_canonical_text() {
    let src = load("phase5/universe.universe");
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let want = print_universe(&u.coding);
    let csr = csr_from_universe(&u.coding);
    let back = coding_from_csr(&csr);
    assert_eq!(print_universe(&back), want);
}

#[test]
fn ordered_member_order_survives_csr_and_unordered_sorts() {
    let ordered = match parse_universe(&load("phase5/ordered.universe")) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let csr = csr_from_universe(&ordered.coding);
    assert_eq!(csr.link_orders, vec![Order::Ordered]);
    assert_eq!(csr.members.len(), 2);
    assert_eq!(csr.instances[csr.members[0].instance as usize], "sum");
    assert_eq!(csr.instances[csr.members[1].instance as usize], "scale");
    let back = coding_from_csr(&csr);
    assert_eq!(back.links[0].members[0].instance, "sum");
    assert_eq!(back.links[0].members[1].instance, "scale");

    let mixed = match parse_universe(&load("phase5/universe.universe")) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let text = print_universe(&coding_from_csr(&csr_from_universe(&mixed.coding)));
    let e0 = match text.find("link e0 order ordered") {
        Some(i) => &text[i..],
        None => panic!("e0"),
    };
    let calc = match e0.find("calc.sum@2") {
        Some(i) => i,
        None => panic!("calc"),
    };
    let units = match e0.find("units.scale@0") {
        Some(i) => i,
        None => panic!("units"),
    };
    assert!(calc < units);
}
