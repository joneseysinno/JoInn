//! Insert a body under its computed coding hash.

use joinn_dna::{Body, Cell, hash};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use super::{BodyStore, Entry};
use crate::assay::check_declarations;
use crate::bind::bind;
use crate::universe::{BodyBinding, Universe, UniverseCoding, UniverseRegulatory};

impl BodyStore {
    /// Compute the coding hash as the key. Refuses a second body with the same
    /// coding hash and a different regulatory region, naming both sources.
    /// A body that carries a declaration is assayed alone, as alias `body`,
    /// with `cells`, and refused when the declaration does not hold.
    pub fn insert(
        &mut self,
        body: Body,
        cells: BTreeMap<Hash, Cell>,
        source: &str,
    ) -> Verdict<Hash> {
        let id = hash(&body.coding);
        if !body.coding.declarations.is_empty() {
            let mut alone = BodyStore::default();
            alone.entries.insert(
                id,
                Entry {
                    body: body.clone(),
                    cells: cells.clone(),
                    source: source.to_owned(),
                },
            );
            let universe = Universe {
                coding: UniverseCoding {
                    codex: 1,
                    declarations: Vec::new(),
                    bodies: vec![BodyBinding {
                        hash: id,
                        alias: "body".to_owned(),
                    }],
                    links: Vec::new(),
                    cross_wires: Vec::new(),
                    grants: BTreeMap::new(),
                    lenses: Vec::new(),
                },
                regulatory: UniverseRegulatory::default(),
            };
            let bound = match bind(&universe, &alone) {
                Verdict::Ok(bound) => bound,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            if let Verdict::Refused(r) =
                check_declarations(&body.coding.declarations, &universe, &bound)
            {
                return Verdict::Refused(r);
            }
        }
        if let Some(prev) = self.entries.get(&id) {
            if prev.body.regulatory != body.regulatory {
                return crate::refuse(format!(
                    "coding hash has distinct faces: {} and {}",
                    prev.source, source
                ));
            }
            return Verdict::Ok(id);
        }
        self.entries.insert(
            id,
            Entry {
                body,
                cells,
                source: source.to_owned(),
            },
        );
        Verdict::Ok(id)
    }
}
