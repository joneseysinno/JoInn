//! A value crosses e0. Observations are descriptions, fire counts, and refusals.

use joinn_dna::{hash, parse_body, parse_cell, Body, Cell};
use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, TextFrame, Value, Verdict};
use joinn_host::{describe, probe, Address};
use joinn_link::{
    assemble_universe, bind_bodies, check_link_types, grant, parse_universe, revoke, Universe,
    UniverseReport, UniverseState, LinkRefusalKind,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn corpus() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn cells() -> BTreeMap<Hash, Cell> {
    let frames = FrameRegistry::phase1();
    let mut out: BTreeMap<Hash, Cell> = BTreeMap::new();
    let mut dirs = vec![corpus()];
    while let Some(dir) = dirs.pop() {
        let rd = match fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        let mut ents: Vec<_> = rd.flatten().collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let path = ent.path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("cell") {
                continue;
            }
            let Ok(src) = fs::read_to_string(&path) else {
                continue;
            };
            if let Verdict::Ok(cell) = parse_cell(&src, &frames) {
                let id = hash(&cell.coding);
                let replace = match out.get(&id) {
                    Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
                    None => true,
                };
                if replace {
                    out.insert(id, cell);
                }
            }
        }
    }
    out
}

fn body_at(rel: &str) -> Body {
    let src = fs::read_to_string(corpus().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
    match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
    }
}

fn supplied() -> BTreeMap<Hash, (Body, BTreeMap<Hash, Cell>)> {
    let all = cells();
    let mut map = BTreeMap::new();
    for rel in [
        "phase2/calculator.body",
        "phase5/units.body",
        "phase5/controls/echo.body",
        "phase5/bus.body",
    ] {
        let body = body_at(rel);
        map.insert(hash(&body.coding), (body, all.clone()));
    }
    map
}

fn universe(rel: &str) -> Universe {
    let src = fs::read_to_string(corpus().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
    match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
    }
}

fn text(s: &str) -> Value {
    match TextFrame::new().canonicalize(Term::text(s)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn int(n: i64) -> Value {
    match IntFrame::new().canonicalize(Term::int(n)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn addr(instance: &str, port: u32) -> Address {
    Address {
        instance: instance.into(),
        port,
    }
}

fn open(u: &Universe, grant_e0: bool) -> UniverseState {
    let bound = match bind_bodies(u, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut state = match UniverseState::new(u, bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    if grant_e0 {
        match grant(state.link_runtime(), u, "e0", "units") {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
    state
}

fn feed_calc(state: &mut UniverseState, a: &str, b: &str) {
    match state.inject("calc", &addr("cli_a", 0), text(a), 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match state.inject("calc", &addr("cli_b", 0), text(b), 1) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn units_fires(reports: &[UniverseReport]) -> usize {
    reports
        .iter()
        .filter(|r| matches!(r, UniverseReport::Fired { body, .. } if body == "units"))
        .count()
}

fn scale_out(state: &UniverseState) -> Option<String> {
    let body = state.body("units").expect("units");
    let d = match describe(body, "scale") {
        Verdict::Ok(d) => d,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    d.ports
        .iter()
        .find(|p| p.position == 2)
        .and_then(|p| p.value.clone())
}

#[test]
fn sixty_crosses_and_unlinked_does_not_fire() {
    let u = universe("phase5/universe.universe");
    let mut state = open(&u, true);
    feed_calc(&mut state, "2", "3");
    match state.inject("units", &addr("scale", 1), int(12), 2) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(units_fires(&reports), 1, "{reports:?}");
    assert_eq!(scale_out(&state).as_deref(), Some("60"));

    let bare = universe("phase51/controls/unlinked.universe");
    let mut quiet = open(&bare, false);
    feed_calc(&mut quiet, "2", "3");
    match quiet.inject("units", &addr("scale", 1), int(12), 2) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let reports = match quiet.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(units_fires(&reports), 0, "{reports:?}");
}

#[test]
fn wrong_hash_names_both_shorts() {
    let u = universe("phase5/controls/wrong_hash.universe");
    let mut map = supplied();
    let calc = body_at("phase2/calculator.body");
    let calc_hash = hash(&calc.coding);
    let cells = map.get(&calc_hash).map(|(_, c)| c.clone()).expect("calc");
    let declared = u.coding.bodies[0].hash;
    map.insert(declared, (calc, cells));
    match bind_bodies(&u, &map) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("calc"), "{}", r.reason);
            assert!(r.reason.contains(&declared.short_hex()), "{}", r.reason);
            assert!(r.reason.contains(&calc_hash.short_hex()), "{}", r.reason);
        }
        Verdict::Ok(_) => panic!("wrong hash must be refused"),
    }
}

#[test]
fn direction_and_frame_name_the_member() {
    let bad = universe("phase5/controls/wrong_direction.universe");
    let bound = match bind_bodies(&bad, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_link_types(&bad, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("calc.cli_a@0"), "{}", r.reason);
            assert!(r.reason.contains("tail"), "{}", r.reason);
            assert!(r.reason.contains("In"), "{}", r.reason);
            assert!(r.reason.contains("Out"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("in-port tail must be refused"),
    }

    let frames = universe("phase5/controls/frame_mismatch.universe");
    let bound = match bind_bodies(&frames, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_link_types(&frames, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("echo.scale@0"), "{}", r.reason);
            assert!(r.reason.contains("Text 1"), "{}", r.reason);
            assert!(r.reason.contains("ℤ 1"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("frame mismatch must be refused"),
    }

    let mut phase5 = universe("phase5/universe.universe");
    let echo = body_at("phase5/controls/echo.body");
    let echo_hash = hash(&echo.coding);
    for binding in &mut phase5.coding.bodies {
        if binding.alias == "units" {
            binding.hash = echo_hash;
        }
    }
    let bound = match bind_bodies(&phase5, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("bind echo by its hash: {}", r.reason),
    };
    match check_link_types(&phase5, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("units.scale@0"), "{}", r.reason);
            assert!(r.reason.contains("Text 1"), "{}", r.reason);
            assert!(r.reason.contains("ℤ 1"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("phase 5 universe on echo must be refused"),
    }
}

#[test]
fn no_such_port_is_not_interior() {
    let u = universe("phase5/controls/no_such_port.universe");
    let bound = match bind_bodies(&u, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match assemble_universe(&u, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("calc.sum@9"), "{}", r.reason);
            assert!(r.reason.contains("no such port"), "{}", r.reason);
            assert!(!r.reason.contains("interior"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("sum@9 must be refused"),
    }
    let transit = universe("phase5/controls/transits.universe");
    let bound = match bind_bodies(&transit, &supplied()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match assemble_universe(&transit, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("interior"), "{}", r.reason);
            assert!(r.reason.contains("cli_a@1 -> sum@0"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("transits must stay interior"),
    }
}

#[test]
fn revoke_stops_the_second_delivery() {
    let u = universe("phase5/universe.universe");
    let mut state = open(&u, true);
    feed_calc(&mut state, "2", "3");
    match state.inject("units", &addr("scale", 1), int(12), 2) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let first = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(units_fires(&first), 1);
    let before = scale_out(&state);
    match revoke(state.link_runtime(), "e0", "units") {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    feed_calc(&mut state, "4", "5");
    match state.inject("units", &addr("scale", 1), int(12), 3) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let second = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let later = units_fires(&second);
    assert_eq!(later, 0, "{second:?}");
    assert_eq!(scale_out(&state), before);
    assert!(
        second.iter().any(|r| matches!(
            r,
            UniverseReport::Link(refusal)
                if refusal.link == "e0" && refusal.kind == LinkRefusalKind::CapabilityNotHeld
        )),
        "{second:?}"
    );
}

#[test]
fn second_sum_stays_home() {
    let u = universe("phase5/universe.universe");
    let mut state = open(&u, true);
    feed_calc(&mut state, "2", "3");
    let _ = state.run();
    feed_calc(&mut state, "2", "3");
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let far = reports.iter().find_map(|r| match r {
        UniverseReport::Link(refusal) => Some(format!("{refusal:?}")),
        UniverseReport::Fired { .. } => None,
    });
    let far = far.expect("a link refusal");
    assert!(!far.contains("join refuse"), "{far}");
    let body = state.body("units").expect("units");
    let seen = match probe(
        body,
        Address {
            instance: "scale".into(),
            port: 0,
        },
    ) {
        Verdict::Ok(d) => d.label,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        seen.contains("join refuse at scale port 0"),
        "{seen}"
    );
}
