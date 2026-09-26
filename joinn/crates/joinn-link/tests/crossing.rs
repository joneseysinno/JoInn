//! A value crosses e0. Observations are descriptions, fire counts, and refusals.

use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, TextFrame, Value, Verdict};
use joinn_host::{Address, describe, describe_refusal, probe};
use joinn_link::{
    BodyStore, LinkRefusalKind, Universe, UniverseReport, UniverseState, assemble_universe, bind,
    check_law4, check_link_types, format_link_refusal, parse_universe, revoke,
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

fn cells() -> BTreeMap<joinn_frame::Hash, Cell> {
    let frames = FrameRegistry::phase1();
    let mut out: BTreeMap<joinn_frame::Hash, Cell> = BTreeMap::new();
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

fn store() -> BodyStore {
    let all = cells();
    let mut store = BodyStore::new();
    for rel in [
        "phase2/calculator.body",
        "phase5/units.body",
        "phase5/controls/echo.body",
        "phase5/bus.body",
    ] {
        let body = body_at(rel);
        match store.insert(body, all.clone(), rel) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        }
    }
    store
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

fn open(u: &Universe) -> UniverseState {
    let bound = match bind(u, &store()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match UniverseState::new(u, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
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
    let body = state
        .body("units")
        .unwrap_or_else(|| panic!("units body missing"));
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
fn body_refusal_is_a_report_and_universe_keeps_running() {
    let u = universe("phase5/universe.universe");
    let mut state = open(&u);
    match state.inject("calc", &addr("cli_a", 0), text("two"), 0) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let first = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        first.iter().any(|r| matches!(
            r,
            UniverseReport::Refused { body, instance }
                if body == "calc" && instance == "cli_a"
        )),
        "{first:?}"
    );
    assert!(
        !first.iter().any(|r| matches!(r, UniverseReport::Link(_))),
        "{first:?}"
    );
    let calc = state
        .body("calc")
        .unwrap_or_else(|| panic!("calc body missing"));
    let reason = calc
        .last_refusal()
        .unwrap_or_else(|| panic!("calc must retain refusal reason"));
    let d = describe_refusal(calc, "cli_a", reason);
    assert!(!d.label.is_empty());

    feed_calc(&mut state, "2", "3");
    match state.inject("units", &addr("scale", 1), int(12), 2) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let second = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(units_fires(&second), 1, "{second:?}");
    assert_eq!(scale_out(&state).as_deref(), Some("60"));

    match state.inject("units", &addr("scale", 1), int(12), 3) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match state.inject("units", &addr("scale", 1), int(12), 4) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let third = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        third.iter().any(|r| matches!(
            r,
            UniverseReport::Refused { body, .. } if body == "units"
        )),
        "{third:?}"
    );
    assert!(
        !third.iter().any(|r| matches!(r, UniverseReport::Link(_))),
        "{third:?}"
    );
}

#[test]
fn sixty_crosses_and_unlinked_does_not_fire() {
    let u = universe("phase5/universe.universe");
    let mut state = open(&u);
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

    let mut bare = universe("phase5/universe.universe");
    bare.coding.links.retain(|l| l.id != "e0");
    bare.coding.grants.remove("e0");
    let mut quiet = open(&bare);
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
fn alias_is_local_binds() {
    let u = universe("phase5/controls/alias_is_local.universe");
    match bind(&u, &store()) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn direction_and_frame_name_the_member() {
    let bad = universe("phase5/controls/wrong_direction.universe");
    let bound = match bind(&bad, &store()) {
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
    let bound = match bind(&frames, &store()) {
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
    let bound = match bind(&phase5, &store()) {
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
    let bound = match bind(&u, &store()) {
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
    let bound = match bind(&transit, &store()) {
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
    let mut state = open(&u);
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
    let mut state = open(&u);
    feed_calc(&mut state, "2", "3");
    let _ = state.run();
    feed_calc(&mut state, "2", "3");
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let far = reports.iter().find_map(|r| match r {
        UniverseReport::Link(refusal) => Some(format_link_refusal(refusal)),
        UniverseReport::Fired { .. } | UniverseReport::Refused { .. } => None,
    });
    let far = far.unwrap_or_else(|| panic!("expected a link refusal in reports"));
    assert!(!far.contains("join refuse"), "{far}");
    let body = state
        .body("units")
        .unwrap_or_else(|| panic!("units body missing"));
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
    assert!(seen.contains("join refuse at scale port 0"), "{seen}");
}

/// Copied from the plan. A test fixture, not a corpus file.
const CHAIN_UNIVERSE: &str = "\
universe {
  codex 1
  bodies {
    body:b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde as calc
    body:2d5fcc96689b26df93fa86ace7525b4820d1bd7f68b46e9ce75ee77d7e2c9fda as units
    body:2d5fcc96689b26df93fa86ace7525b4820d1bd7f68b46e9ce75ee77d7e2c9fda as again
  }
  links {
    link e0 order none {
      calc.sum@2 tail
      units.scale@0 head
    }
    link e1 order none {
      units.scale@2 tail
      again.scale@0 head
    }
  }
  lenses {
    lens function {
      galaxy app {
        system s { calc units again }
      }
    }
  }
}
";

fn open_chain() -> UniverseState {
    let u = match parse_universe(CHAIN_UNIVERSE) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    open(&u)
}

fn inject_ok(state: &mut UniverseState, body: &str, address: Address, value: Value, epoch: u64) {
    match state.inject(body, &address, value, epoch) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn second_hop_reaches_again() {
    let mut state = open_chain();
    feed_calc(&mut state, "2", "3");
    inject_ok(&mut state, "units", addr("scale", 1), int(12), 2);
    inject_ok(&mut state, "again", addr("scale", 1), int(12), 3);
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert!(
        reports.iter().any(|r| matches!(
            r,
            UniverseReport::Fired { body, instance }
                if body == "again" && instance == "scale"
        )),
        "{reports:?}"
    );
}

#[test]
fn again_refusal_is_reported_once() {
    let mut state = open_chain();
    feed_calc(&mut state, "2", "3");
    inject_ok(&mut state, "units", addr("scale", 1), int(12), 2);
    inject_ok(&mut state, "again", addr("scale", 1), int(12), 3);
    inject_ok(&mut state, "again", addr("scale", 1), int(12), 4);
    inject_ok(&mut state, "again", addr("scale", 1), int(12), 5);
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let refused = reports
        .iter()
        .filter(|r| {
            matches!(
                r,
                UniverseReport::Refused { body, instance }
                    if body == "again" && instance == "scale"
            )
        })
        .count();
    assert_eq!(refused, 1, "{reports:?}");
}

#[test]
fn law4_adversary_attempt() {
    let lookup = body_at("phase52/adversary/lookup.body");
    let id = hash(&lookup.coding).to_hex();
    println!("lookup coding hash {id}");
    let all = cells();
    let mut bodies = store();
    match bodies.insert(lookup, all, "phase52/adversary/lookup.body") {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let u = universe("phase52/adversary/lookup.universe");
    match check_law4(&u) {
        Verdict::Ok(()) => println!("Law 4: admitted"),
        Verdict::Refused(r) => println!("Law 4: {}", r.reason),
    }
    let bound = match bind(&u, &bodies) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_link_types(&u, &bound) {
        Verdict::Ok(()) => println!("typing: admitted"),
        Verdict::Refused(r) => println!("typing: {}", r.reason),
    }
    match assemble_universe(&u, &bound) {
        Verdict::Ok(()) => println!("assembly: admitted"),
        Verdict::Refused(r) => println!("assembly: {}", r.reason),
    }
    match check_link_types(&u, &bound) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("units.scale@1"), "{}", r.reason);
            assert!(r.reason.contains("tail"), "{}", r.reason);
            assert!(r.reason.contains("In"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("reading the factor out of an in-port must be refused"),
    }
}

fn store_with_asker() -> BodyStore {
    let all = cells();
    let mut bodies = store();
    let asker = body_at("phase52/adversary/asker.body");
    match bodies.insert(asker, all, "phase52/adversary/asker.body") {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    bodies
}

fn open_ask(u: &Universe) -> UniverseState {
    let bound = match bind(u, &store_with_asker()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match UniverseState::new(u, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

fn feed_question(state: &mut UniverseState, factor: i64, mut epoch: u64) -> u64 {
    let steps = [
        ("lookup", "question", 0u32, int(1)),
        ("lookup", "question", 1, int(1)),
        ("units", "scale", 1, int(factor)),
        ("lookup", "answer", 1, int(1)),
    ];
    for (body, instance, port, value) in steps {
        match state.inject(body, &addr(instance, port), value, epoch) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        epoch = epoch.saturating_add(1);
    }
    epoch
}

fn answer_at(state: &UniverseState) -> String {
    let body = state
        .body("lookup")
        .unwrap_or_else(|| panic!("lookup body missing"));
    let d = match describe(body, "answer") {
        Verdict::Ok(d) => d,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    d.ports
        .iter()
        .find(|p| p.position == 2)
        .and_then(|p| p.value.clone())
        .unwrap_or_else(|| panic!("answer@2 empty"))
}

#[test]
fn law4_adversary_asks_by_one() {
    let asker = body_at("phase52/adversary/asker.body");
    let id = hash(&asker.coding).to_hex();
    let u = universe("phase52/adversary/ask.universe");
    assert!(
        u.coding
            .bodies
            .iter()
            .any(|b| b.alias == "lookup" && b.hash.to_hex() == id),
        "ask.universe must name asker {id}"
    );
    match check_law4(&u) {
        Verdict::Ok(()) => println!("Law 4: admitted"),
        Verdict::Refused(r) => println!("Law 4: {}", r.reason),
    }
    let bound = match bind(&u, &store_with_asker()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_link_types(&u, &bound) {
        Verdict::Ok(()) => println!("typing: admitted"),
        Verdict::Refused(r) => println!("typing: {}", r.reason),
    }
    match assemble_universe(&u, &bound) {
        Verdict::Ok(()) => println!("assembly: admitted"),
        Verdict::Refused(r) => println!("assembly: {}", r.reason),
    }
    match check_law4(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("Law 4 must admit ask.universe: {}", r.reason),
    }
    match check_link_types(&u, &bound) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("typing must admit ask.universe: {}", r.reason),
    }
    match assemble_universe(&u, &bound) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("assembly must admit ask.universe: {}", r.reason),
    }

    let mut state = open_ask(&u);
    let mut epoch = feed_question(&mut state, 12, 0);
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let answer = answer_at(&state);
    let round0 = format!("round 0 factor 12: {reports:?} answer {answer}");
    println!("{round0}");
    assert_eq!(
        round0,
        "round 0 factor 12: [Fired { body: \"lookup\", instance: \"question\" }, Fired { body: \"units\", instance: \"scale\" }, Fired { body: \"lookup\", instance: \"answer\" }] answer 12"
    );

    let _ = feed_question(&mut state, 5, epoch);
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let answer = answer_at(&state);
    let round1 = format!("round 1 factor 5: {reports:?} answer {answer}");
    println!("{round1}");
    assert_eq!(
        round1,
        "round 1 factor 5: [Fired { body: \"lookup\", instance: \"question\" }, Fired { body: \"units\", instance: \"scale\" }, Fired { body: \"lookup\", instance: \"answer\" }] answer 5"
    );

    let mut once = open_ask(&u);
    epoch = feed_question(&mut once, 12, 0);
    let _ = feed_question(&mut once, 5, epoch);
    let reports = match once.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let once_line = format!("one run: {reports:?}");
    println!("{once_line}");
    assert!(
        !reports.iter().any(|r| matches!(
            r,
            UniverseReport::Fired { body, instance }
                if body == "lookup" && instance == "answer"
        )),
        "{once_line}"
    );
    assert_eq!(
        once_line,
        "one run: [Refused { body: \"lookup\", instance: \"answer\" }, Refused { body: \"units\", instance: \"scale\" }]"
    );
}
