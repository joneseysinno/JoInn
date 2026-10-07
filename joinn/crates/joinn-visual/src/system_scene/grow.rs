//! Grow a system scene with nothing accepted yet.

use std::collections::BTreeMap;

use joinn_dna::{Cell, Contact, System};
use joinn_frame::{Hash, Verdict};

use super::SystemScene;

impl SystemScene {
    /// Size 0: `waiting` empty boxes, the lasso, and the response's identity.
    pub fn grow(
        system: &System,
        contacts: &BTreeMap<Hash, Contact>,
        cells: &BTreeMap<Hash, Cell>,
        waiting: u32,
    ) -> Verdict<SystemScene> {
        SystemScene::regrow(system, contacts, cells, waiting, &[])
    }
}
