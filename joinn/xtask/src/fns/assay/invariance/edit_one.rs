//! Edit 1 hands the assay the edited body, not the one still in the store.

use joinn_dna::hash;
use joinn_frame::Verdict;
use joinn_link::{BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, bind};
use std::collections::BTreeMap;

use crate::fns::mutate::{neutral, reparse, reprint};
use crate::fns::subject::Subject;

/// The universe edit 1 assays, the store that holds what changed, and whether a name was added.
pub(crate) struct EditOne {
    /// Universe passed to the assay.
    pub universe: Universe,
    /// A fresh store when the subject is a body. `None` means the caller's store.
    pub fresh: Option<BodyStore>,
    /// True when `neutral` had nothing to edit and the harness inserted a name.
    pub added_name: bool,
}

/// Build the subject edit 1 assays. A body goes into a fresh store with the original cell map.
pub(crate) fn edit_one(subject: &Subject, store: &BodyStore) -> Result<EditOne, String> {
    let (edited, added_name) = if let Some(edited) = neutral(subject) {
        (edited, false)
    } else {
        let mut next = subject.clone();
        match &mut next {
            Subject::Body(body) => {
                let mut instances = Vec::new();
                for entry in &body.coding.genome {
                    for name in &entry.instances {
                        instances.push(name.clone());
                    }
                }
                instances.sort();
                let Some(first) = instances.into_iter().next() else {
                    return Err(
                        "edit 1 found no instance to name; acceptance is a genome instance".into(),
                    );
                };
                body.regulatory.names.insert(first, "neutral".to_string());
            }
            Subject::Universe(u) => {
                let mut aliases: Vec<String> =
                    u.coding.bodies.iter().map(|b| b.alias.clone()).collect();
                aliases.sort();
                let Some(first) = aliases.into_iter().next() else {
                    return Err("edit 1 found no alias to name; acceptance is a bound body".into());
                };
                u.regulatory.names.insert(first, "neutral".to_string());
            }
            _ => {
                return Err(
                    "edit 1 expected a body or a universe; acceptance is one of those kinds".into(),
                );
            }
        }
        let text = reprint(&next);
        match reparse(subject, &text) {
            Verdict::Ok(parsed) => (parsed, true),
            Verdict::Refused(r) => return Err(r.reason),
        }
    };

    let wrap = |body: &joinn_dna::Body| Universe {
        coding: UniverseCoding {
            codex: 1,
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

    // Reprint sorts genome instances. Piece order follows that list, so the
    // inserted value keeps the coding that was assayed and takes only the
    // regulatory region from the reprint.
    match &edited {
        Subject::Body(edited_body) => {
            let Subject::Body(original) = subject else {
                return Err("edit 1 changed a universe into a body".into());
            };
            let mut inserted = original.clone();
            inserted.regulatory = edited_body.regulatory.clone();
            let bound = match bind(&wrap(original), store) {
                Verdict::Ok(bound) => bound,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let Some((_, cells)) = bound.get("body") else {
                return Err("edit 1 bound no body alias; acceptance is alias body".into());
            };
            let mut fresh = BodyStore::new();
            match fresh.insert(inserted.clone(), cells.clone(), "edit 1") {
                Verdict::Ok(_) => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
            Ok(EditOne {
                universe: wrap(&inserted),
                fresh: Some(fresh),
                added_name,
            })
        }
        Subject::Universe(edited_u) => {
            let Subject::Universe(original) = subject else {
                return Err("edit 1 changed a body into a universe".into());
            };
            let mut universe = original.clone();
            universe.regulatory = edited_u.regulatory.clone();
            Ok(EditOne {
                universe,
                fresh: None,
                added_name,
            })
        }
        _ => Err("edit 1 changed the kind".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::edit_one;
    use crate::fns::subject::Subject;
    use joinn_dna::parse_body;
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_link::{BodyStore, bind};
    use std::collections::BTreeMap;

    #[test]
    fn calculator_edit_one_binds_a_different_regulatory_region() {
        let frames = FrameRegistry::phase1();
        let text = include_str!("../../../../../corpus/phase2/calculator.body");
        let body = match parse_body(text, &frames) {
            Verdict::Ok(body) => body,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut store = BodyStore::new();
        match store.insert(body.clone(), BTreeMap::new(), "phase2/calculator.body") {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        let subject = Subject::Body(body.clone());
        let edit = match edit_one(&subject, &store) {
            Ok(edit) => edit,
            Err(e) => panic!("{e}"),
        };
        let Some(fresh) = &edit.fresh else {
            panic!("calculator edit 1 must assay a fresh store");
        };
        let bound = match bind(&edit.universe, fresh) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let Some((got, _)) = bound.get("body") else {
            panic!("edit 1 did not bind alias body");
        };
        assert_ne!(got.regulatory, body.regulatory);
    }
}
