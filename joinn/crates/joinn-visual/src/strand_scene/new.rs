//! Draw a strand beside its partner.

use std::collections::BTreeSet;

use joinn_codex::meanings;
use joinn_frame::Verdict;
use joinn_strand::Strand;

use super::StrandScene;
use super::pieces::strand_pieces;
use crate::dna_draw::write_pieces;
use crate::tables::Tables;

impl StrandScene {
    /// `partner` is the strand the view pairs with `strand`, read tail first;
    /// the reader's partner (`strand.partner()`) never glows. Every row is
    /// pending until the first draw.
    pub fn new(strand: &Strand, partner: &Strand) -> Verdict<StrandScene> {
        let (pieces, extent) = match strand_pieces(strand, partner) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut tables = Tables::empty();
        let written = match write_pieces(&mut tables, extent, &pieces) {
            Verdict::Ok(w) => w,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        Verdict::Ok(StrandScene {
            hash: strand.identity(),
            codons: meanings(strand).len(),
            tables,
            pending: written.into_iter().collect::<BTreeSet<_>>(),
        })
    }
}
