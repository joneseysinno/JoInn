//! Gate 4 helper: the assay report of a parsed body or universe subject.

use joinn_frame::Verdict;
use joinn_link::{
    AssayReport, BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, assay, bind,
};
use std::collections::BTreeMap;

use super::corpus_store::corpus_store;
use super::subject::Subject;

/// A body is assayed alone as alias `body`; a universe against the corpus store.
pub(crate) fn g4_report(subject: &Subject) -> Option<AssayReport> {
    let (cells, store) = corpus_store().ok()?;
    match subject {
        Subject::Body(body) => {
            let mut alone = BodyStore::new();
            let Verdict::Ok(id) = alone.insert(body.clone(), cells.clone(), "gate 4 subject")
            else {
                return None;
            };
            let universe = Universe {
                coding: UniverseCoding {
                    codex: 1,
                    declarations: Vec::new(),
                    bodies: vec![BodyBinding {
                        hash: id,
                        alias: "body".to_string(),
                    }],
                    links: Vec::new(),
                    cross_wires: Vec::new(),
                    grants: BTreeMap::new(),
                    lenses: Vec::new(),
                },
                regulatory: UniverseRegulatory::default(),
            };
            let Verdict::Ok(bound) = bind(&universe, &alone) else {
                return None;
            };
            match assay(&universe, &bound) {
                Verdict::Ok(report) => Some(report),
                Verdict::Refused(_) => None,
            }
        }
        Subject::Universe(universe) => {
            let Verdict::Ok(bound) = bind(universe, store) else {
                return None;
            };
            match assay(universe, &bound) {
                Verdict::Ok(report) => Some(report),
                Verdict::Refused(_) => None,
            }
        }
        _ => None,
    }
}
