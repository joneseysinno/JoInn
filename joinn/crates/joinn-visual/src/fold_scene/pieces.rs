//! A chromosome at one floor as pieces.

use std::collections::BTreeMap;

use joinn_codex::meanings;
use joinn_frame::Verdict;
use joinn_tower::{Chromosome, level_filter, point};

use super::gene_pieces::gene_pieces;
use super::{MARGIN, NODE_PITCH, RING_RADIUS, SEGMENT_HALF, SILENT_HALF};
use crate::dna_draw::{Piece, meaning_style, owner};
use crate::refuse::refuse;
use crate::tables::{STYLE_RING, STYLE_SILENT};

/// cos and sin of 45° × j in thousandths: a ring is an octagon.
const OCTAGON: [(i64, i64); 8] = [
    (1000, 0),
    (707, 707),
    (0, 1000),
    (-707, 707),
    (-1000, 0),
    (-707, -707),
    (0, -1000),
    (707, -707),
];

/// In draw order: the kept segments, the rings, the opened genes. Floor `f`
/// keeps the segments of level ≥ `f` (the tower's level filter); kept segment
/// `j` runs from node `j` to node `j + 1` of the order `f` floors down, each
/// node at the centre of the block of floor-0 nodes it stands for, so the
/// picture keeps its size. Segment `s` is owned by `[1, 0, CODON_TAG | s]`
/// when it holds a codon (codon k sits on segment k), `[1, 1, …]` when silent;
/// node `d`'s ring by `[1, 2, CODON_TAG | d]`; its opened gene by `[1, 3, …]`.
/// North is up. The extent is in sixteenths.
pub(super) fn fold_pieces(
    c: &Chromosome,
    floor: u32,
    opened: &BTreeMap<u64, Chromosome>,
) -> Verdict<(Vec<Piece>, (i64, i64))> {
    let order = c.order();
    if floor > order {
        return refuse(format!(
            "fold view: floor {floor} is above order {order}'s top floor; acceptance is a floor from 0 to {order}"
        ));
    }
    let down = order - floor;
    let unit = NODE_PITCH << floor;
    let side = 1u64 << down;
    let inset = MARGIN + (unit - NODE_PITCH) / 2;
    let at = |j: u64| {
        let p = point(down, j);
        (
            inset + p.x as i64 * unit,
            inset + (side - 1 - p.y) as i64 * unit,
        )
    };
    let read = meanings(c.strand());
    let all: Vec<u64> = (0..c.segments()).collect();
    let mut pieces = Vec::new();
    for (j, s) in (0u64..).zip(level_filter(&all, floor)) {
        let (part, half, style) = match read.get(s as usize) {
            Some((_, meaning)) => (0, SEGMENT_HALF, meaning_style(*meaning)),
            None => (1, SILENT_HALF, STYLE_SILENT),
        };
        let Some(o) = owner(part, s) else {
            return refuse(format!(
                "fold view: segment {s} does not fit an ID; acceptance is a smaller order"
            ));
        };
        pieces.push(Piece {
            a: at(j),
            b: at(j + 1),
            half,
            style,
            owner: o,
        });
    }
    let node_at = |d: u64| at(d >> (2 * floor));
    for d in c.genes().keys() {
        let Some(o) = owner(2, *d) else {
            return refuse(format!(
                "fold view: node {d} does not fit an ID; acceptance is a smaller order"
            ));
        };
        let (x, y) = node_at(*d);
        let corner = |k: usize| {
            let (cx, cy) = OCTAGON[k % 8];
            (x + RING_RADIUS * cx / 1000, y + RING_RADIUS * cy / 1000)
        };
        pieces.extend((0..8).map(|k| Piece {
            a: corner(k),
            b: corner(k + 1),
            half: 3,
            style: STYLE_RING,
            owner: o,
        }));
    }
    for (d, gene) in opened {
        let Some(o) = owner(3, *d) else {
            return refuse(format!(
                "fold view: node {d} does not fit an ID; acceptance is a smaller order"
            ));
        };
        let (x, y) = node_at(*d);
        let origin = (x + RING_RADIUS + 6, y + RING_RADIUS + 6);
        match gene_pieces(gene, origin, o) {
            Verdict::Ok(p) => pieces.extend(p),
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    let span = 2 * MARGIN + ((1i64 << order) - 1) * NODE_PITCH + NODE_PITCH;
    Verdict::Ok((pieces, (span, span)))
}
