//! Draw the fold at a floor with some genes opened.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_tower::Chromosome;

use super::FoldScene;
use super::pieces::fold_pieces;
use crate::dna_draw::write_pieces;

impl FoldScene {
    /// Every row is written and only rows whose bytes changed are pending. A
    /// refusal writes nothing and changes nothing.
    pub(super) fn redraw(&mut self, floor: u32, opened: BTreeMap<u64, Chromosome>) -> Verdict<()> {
        let (pieces, extent) = match fold_pieces(&self.chromosome, floor, &opened) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut tables = self.tables.clone();
        let written = match write_pieces(&mut tables, extent, &pieces) {
            Verdict::Ok(w) => w,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        self.tables = tables;
        self.pending.extend(written);
        self.floor = floor;
        self.opened = opened;
        Verdict::Ok(())
    }
}
