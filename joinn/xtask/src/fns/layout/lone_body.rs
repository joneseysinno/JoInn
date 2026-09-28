//! Bind one body alone, as the assay does, and hand back the body with its cells.

use joinn_dna::{Body, Cell, hash, parse_body};
use joinn_frame::{CheckId, FrameRegistry, Hash, Refusal, Verdict};
use joinn_link::{BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, bind};
use std::collections::BTreeMap;

/// Parse, store and bind `text` as the lone body `body`. Any refusal is returned.
pub(super) fn lone_body(
    text: &str,
    source: &str,
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<(Body, BTreeMap<Hash, Cell>)> {
    let body = match parse_body(text, &FrameRegistry::phase1()) {
        Verdict::Ok(body) => body,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let universe = Universe {
        coding: UniverseCoding {
            codex: 1,
            declarations: Vec::new(),
            bodies: vec![BodyBinding {
                hash: hash(&body.coding),
                alias: "body".to_string(),
            }],
            links: Vec::new(),
            cross_wires: Vec::new(),
            grants: BTreeMap::new(),
            lenses: Vec::new(),
        },
        regulatory: UniverseRegulatory::default(),
    };
    let mut store = BodyStore::new();
    match store.insert(body, cells.clone(), source) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let bound = match bind(&universe, &store) {
        Verdict::Ok(bound) => bound,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    match bound.get("body") {
        Some((body, cells)) => Verdict::Ok((body.clone(), cells.clone())),
        None => Verdict::Refused(Refusal::structural(
            CheckId::Other,
            format!("{source}: bound no alias body; acceptance is the lone body bound as body"),
        )),
    }
}
