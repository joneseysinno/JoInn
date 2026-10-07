//! The owners a whole-system view may show: its cut.

use std::collections::BTreeSet;

use super::SystemScene;
use crate::pick::PORT_TAG;
use crate::tables::LIVE;

impl SystemScene {
    /// Read from the tables, not the shapes: the surface, every live cell and
    /// port, and the owner of every live stroke (the force for the lasso, the
    /// response's out-port for the count). A system view fitted to its
    /// extent shows all of them.
    pub fn allows(&self) -> BTreeSet<[u32; 4]> {
        let t = &self.tables;
        let live = |flags: u32| flags & LIVE != 0;
        let mut out = BTreeSet::new();
        for (slot, b) in (0u32..).zip(&t.body) {
            if live(b.flags) {
                out.insert([slot + 1, 0, 0, b.generation]);
            }
        }
        for (slot, c) in (0u32..).zip(&t.cell) {
            if live(c.flags) {
                out.insert([c.body + 1, slot + 1, 0, c.generation]);
            }
        }
        for p in t.port.iter().filter(|p| live(p.flags)) {
            if let Some(c) = t.cell.get(p.cell as usize) {
                out.insert([c.body + 1, p.cell + 1, PORT_TAG | p.position, c.generation]);
            }
        }
        for s in t.stroke.iter().filter(|s| live(s.flags)) {
            out.insert([s.owner[0], s.owner[1], s.owner[2], s.generation]);
        }
        out
    }
}
