//! Two derivations, one truth: on every `.contact` in the corpus, the contact's
//! own surface equals the surface of the body `lower` derives from it.

use joinn_dna::{Cell, Direction, hash, keep_first_with_alleles, parse_cell, parse_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{contact_surface, lower, surface};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

fn files(ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut dirs = vec![corpus()];
    while let Some(dir) = dirs.pop() {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        let mut ents: Vec<_> = rd.flatten().collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let path = ent.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn cells() -> BTreeMap<Hash, Cell> {
    let frames = FrameRegistry::phase1();
    let mut out: BTreeMap<Hash, Cell> = BTreeMap::new();
    for path in files("cell") {
        let Ok(src) = fs::read_to_string(&path) else {
            continue;
        };
        if let Verdict::Ok(cell) = parse_cell(&src, &frames) {
            keep_first_with_alleles(&mut out, hash(&cell.coding), cell);
        }
    }
    out
}

#[test]
fn contact_surface_equals_the_surface_of_the_lowered_body_on_every_contact() {
    let frames = FrameRegistry::phase1();
    let cells = cells();
    let contacts = files("contact");
    assert!(!contacts.is_empty(), "no .contact in the corpus");
    for path in &contacts {
        let src = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
        let contact = match parse_contact(&src, &frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{path:?}: {}", r.reason),
        };
        let own = match contact_surface(&contact, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{path:?}: {}", r.reason),
        };
        let lowered = match lower(&contact, &cells, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{path:?}: {}", r.reason),
        };
        let derived = match surface(&lowered, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{path:?}: {}", r.reason),
        };
        assert_eq!(own, derived, "{path:?}");
        let intents = |set: &BTreeSet<joinn_link::BoundaryPort>| -> BTreeSet<String> {
            set.iter()
                .filter(|p| p.direction == Direction::In)
                .map(|p| p.address.printed())
                .collect()
        };
        assert_eq!(intents(&own), intents(&derived), "{path:?}");
    }
}

#[test]
fn the_calculator_contact_surface_is_three_ports() {
    let frames = FrameRegistry::phase1();
    let src = fs::read_to_string(corpus().join("phase7").join("calculator.contact"))
        .unwrap_or_else(|e| panic!("{e}"));
    let contact = match parse_contact(&src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let got: Vec<String> = match contact_surface(&contact, &cells()) {
        Verdict::Ok(s) => s
            .iter()
            .map(|p| {
                let d = match p.direction {
                    Direction::In => "in",
                    Direction::Out => "out",
                };
                format!("{} {d} {}", p.address.printed(), p.frame)
            })
            .collect(),
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(
        got,
        ["cli_a@0 in Text 1", "cli_b@0 in Text 1", "sum@2 out ℤ 1"]
    );
}
