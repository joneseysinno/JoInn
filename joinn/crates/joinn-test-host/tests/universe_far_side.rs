//! Universe run under the test host: far side and host intents.

use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Term, Verdict};
use joinn_host::Address;
use joinn_link::{BodyStore, LinkRefusal, LinkRefusalKind, bind, parse_universe};
use joinn_test_host::{RawEvent, run_universe};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn load_cells(root: &Path) -> BTreeMap<Hash, Cell> {
    let frames = FrameRegistry::phase1();
    let mut cells: BTreeMap<Hash, Cell> = BTreeMap::new();
    for dir in ["phase0", "phase21", "phase22", "phase2"] {
        let path = root.join(dir);
        let Ok(rd) = fs::read_dir(&path) else {
            continue;
        };
        let mut ents: Vec<_> = rd.flatten().collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let file = ent.path();
            if file.extension().and_then(|e| e.to_str()) != Some("cell") {
                continue;
            }
            let Ok(src) = fs::read_to_string(&file) else {
                continue;
            };
            let Verdict::Ok(cell) = parse_cell(&src, &frames) else {
                continue;
            };
            let id = hash(&cell.coding);
            let replace = match cells.get(&id) {
                Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
                None => true,
            };
            if replace {
                cells.insert(id, cell);
            }
        }
    }
    cells
}

fn load_store() -> BodyStore {
    let root = corpus();
    let frames = FrameRegistry::phase1();
    let cells = load_cells(&root);
    let mut store = BodyStore::new();
    for rel in ["phase2/calculator.body", "phase5/units.body"] {
        let path = root.join(rel);
        let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        let body: Body = match parse_body(&src, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        };
        let source = path.display().to_string();
        match store.insert(body, cells.clone(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{rel}: {}", r.reason),
        }
    }
    store
}

fn crossing_events(units_alias: &str) -> Vec<(String, RawEvent)> {
    vec![
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_a".into(),
                    port: 0,
                },
                term: Term::text("2"),
            },
        ),
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_b".into(),
                    port: 0,
                },
                term: Term::text("3"),
            },
        ),
        (
            units_alias.into(),
            RawEvent {
                address: Address {
                    instance: "scale".into(),
                    port: 1,
                },
                term: Term::int(12),
            },
        ),
    ]
}

#[test]
fn units_host_intent_set_is_unlinked_factor() {
    let root = corpus();
    let src = fs::read_to_string(root.join("phase5").join("universe.universe"))
        .unwrap_or_else(|e| panic!("{e}"));
    let universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let store = load_store();
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let cap = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        vec![crossing_events("units")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut want = std::collections::BTreeSet::new();
    want.insert(Address {
        instance: "scale".into(),
        port: 1,
    });
    assert_eq!(cap.intent_set["units"], want);
    assert!(cap.far_side.is_empty(), "{:?}", cap.far_side);
}

#[test]
fn rename_link_leaves_descriptions() {
    let root = corpus();
    let src = fs::read_to_string(root.join("phase5").join("universe.universe"))
        .unwrap_or_else(|e| panic!("{e}"));
    let mut universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let store = load_store();
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let before = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        vec![crossing_events("units")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    for link in &mut universe.coding.links {
        if link.id == "e0" {
            link.id = "e1".into();
        }
    }
    if let Some(body) = universe.coding.grants.remove("e0") {
        universe.coding.grants.insert("e1".into(), body);
    }
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let after = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        vec![crossing_events("units")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(before.descriptions, after.descriptions);
}

#[test]
fn rename_alias_leaves_descriptions() {
    let root = corpus();
    let src = fs::read_to_string(root.join("phase5").join("universe.universe"))
        .unwrap_or_else(|e| panic!("{e}"));
    let universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let store = load_store();
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let before = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        vec![crossing_events("units")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut renamed = universe;
    for binding in &mut renamed.coding.bodies {
        if binding.alias == "units" {
            binding.alias = "meters".into();
        }
    }
    for link in &mut renamed.coding.links {
        for member in &mut link.members {
            if member.body == "units" {
                member.body = "meters".into();
            }
        }
    }
    for holder in renamed.coding.grants.values_mut() {
        if holder == "units" {
            *holder = "meters".into();
        }
    }
    for lens in &mut renamed.coding.lenses {
        for galaxy in &mut lens.galaxies {
            for system in &mut galaxy.systems {
                for body in &mut system.bodies {
                    if body == "units" {
                        *body = "meters".into();
                    }
                }
            }
        }
    }
    let bound = match bind(&renamed, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let after = match run_universe(
        &renamed,
        bound,
        joinn_prim::sealed_natives(),
        vec![crossing_events("meters")],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(before.descriptions, after.descriptions);
    let mut want = BTreeSet::new();
    want.insert(Address {
        instance: "scale".into(),
        port: 1,
    });
    assert_eq!(after.intent_set["meters"], want);
}

fn calc_round() -> Vec<(String, RawEvent)> {
    vec![
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_a".into(),
                    port: 0,
                },
                term: Term::text("2"),
            },
        ),
        (
            "calc".into(),
            RawEvent {
                address: Address {
                    instance: "cli_b".into(),
                    port: 0,
                },
                term: Term::text("3"),
            },
        ),
    ]
}

#[test]
fn double_delivery_far_side_is_one_refusal() {
    let root = corpus();
    let src = fs::read_to_string(root.join("phase5").join("universe.universe"))
        .unwrap_or_else(|e| panic!("{e}"));
    let universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let store = load_store();
    let bound = match bind(&universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let cap = match run_universe(
        &universe,
        bound,
        joinn_prim::sealed_natives(),
        vec![calc_round(), calc_round()],
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(
        cap.far_side,
        vec![LinkRefusal {
            link: "e0".into(),
            body: "units".into(),
            member: Address {
                instance: "scale".into(),
                port: 0,
            },
            kind: LinkRefusalKind::Refused,
        }]
    );
}
