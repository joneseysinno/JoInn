//! Open a system: grow its scene at size 0 and fit it.

use std::collections::BTreeMap;

use joinn_dna::{Cell, Contact, System};
use joinn_frame::{Hash, Verdict};
use joinn_visual::{Camera, SystemScene};

use super::Grower;

impl Grower {
    /// `waiting` empty boxes, the lasso and the count, fitted to 1280×720 until
    /// the window says otherwise. Nothing is selected.
    pub fn open(
        system: &System,
        contacts: &BTreeMap<Hash, Contact>,
        cells: &BTreeMap<Hash, Cell>,
        waiting: u32,
    ) -> Result<Grower, String> {
        let scene = match SystemScene::grow(system, contacts, cells, waiting) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let camera = match Camera::from_fit(&scene.fit(1280, 720)) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        Ok(Grower {
            scene,
            camera,
            selected: None,
            buffer: String::new(),
            confirm: None,
        })
    }
}
