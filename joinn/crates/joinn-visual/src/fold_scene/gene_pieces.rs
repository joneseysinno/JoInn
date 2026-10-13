//! An opened gene, drawn small.

use joinn_codex::meanings;
use joinn_frame::Verdict;
use joinn_tower::{Chromosome, point, short_hash};

use super::GENE_SPAN;
use crate::dna_draw::{Piece, label, meaning_style};
use crate::font::TextSize;
use crate::tables::STYLE_SILENT;

/// A gene's hash: half a label's size, cap height half a layout unit.
const GENE_LABEL: TextSize = TextSize {
    unit: 1,
    half_width: 1,
};

/// The gene's whole curve with its top-left node at `origin`, spanning at most
/// `GENE_SPAN` sixteenths, tinted as the fold is; an order-0 gene is a dot.
/// Its hash, cut to twelve hex digits, is written under it. Every piece is
/// owned by `owner`.
pub(super) fn gene_pieces(
    gene: &Chromosome,
    origin: (i64, i64),
    owner: [u32; 3],
) -> Verdict<Vec<Piece>> {
    let order = gene.order();
    let step = (GENE_SPAN >> order).max(2);
    let side = 1u64 << order;
    let at = |d: u64| {
        let p = point(order, d);
        (
            origin.0 + p.x as i64 * step,
            origin.1 + (side - 1 - p.y) as i64 * step,
        )
    };
    let read = meanings(gene.strand());
    let mut pieces: Vec<Piece> = (0..gene.segments())
        .map(|s| {
            let (half, style) = match read.get(s as usize) {
                Some((_, meaning)) => (3, meaning_style(*meaning)),
                None => (1, STYLE_SILENT),
            };
            Piece {
                a: at(s),
                b: at(s + 1),
                half,
                style,
                owner,
            }
        })
        .collect();
    if pieces.is_empty() {
        pieces.push(Piece {
            a: origin,
            b: origin,
            half: 4,
            style: STYLE_SILENT,
            owner,
        });
    }
    let span = (side as i64 - 1) * step;
    match label(
        &short_hash(&gene.identity()),
        origin.0 + span / 2,
        origin.1 + span + 8,
        GENE_LABEL,
        owner,
    ) {
        Verdict::Ok(name) => pieces.extend(name),
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    Verdict::Ok(pieces)
}
