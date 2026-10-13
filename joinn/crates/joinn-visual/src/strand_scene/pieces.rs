//! A strand and its partner as pieces.

use joinn_codex::{Meaning, meanings};
use joinn_frame::Verdict;
use joinn_strand::Strand;

use super::helix::helix;
use super::{AXIS, HELIX_RADIUS, LETTER_PITCH, MARGIN};
use crate::dna_draw::{Piece, label, meaning_style, owner};
use crate::font::LABEL_SIZE;
use crate::refuse::refuse;
use crate::tables::{STYLE_BACKBONE, STYLE_END_CAP, STYLE_GLOW, STYLE_LETTER_A};

/// In draw order: each codon's tint, the end caps, the glows, both backbones,
/// the rungs (half a rung per letter, in its letter's colour, meeting on the
/// axis), then the names. Letter `i` pairs with partner letter `n − 1 − i`;
/// a pair whose partner letter is not the strand letter's partner glows. A
/// piece of codon `k` is owned by `[1, 0, CODON_TAG | k]`; letters past the
/// last whole codon by the surface. The extent is in sixteenths.
pub(super) fn strand_pieces(
    strand: &Strand,
    partner: &Strand,
) -> Verdict<(Vec<Piece>, (i64, i64))> {
    let (sense, other) = (strand.letters(), partner.letters());
    let n = sense.len();
    if other.len() != n {
        return refuse(format!(
            "strand view: a partner of {} letters beside a strand of {n}; acceptance is a partner as long as its strand",
            other.len()
        ));
    }
    let read = meanings(strand);
    let mut owners = Vec::with_capacity(read.len());
    for k in 0..read.len() {
        let Some(o) = owner(0, k as u64) else {
            return refuse(format!(
                "strand view: codon {k} does not fit an ID; acceptance is a shorter strand"
            ));
        };
        owners.push(o);
    }
    let owner_of = |i: usize| owners.get(i / 3).copied().unwrap_or([1, 0, 0]);
    let at = |i: usize| helix(i as i64);
    let rung_x = |i: usize| (at(i).0.0 + at(i).1.0) / 2;
    let mut pieces = Vec::new();
    let mut caps = Vec::new();
    let mut names = Vec::new();
    for (k, (_, meaning)) in read.iter().enumerate() {
        let (first, last) = (rung_x(3 * k), rung_x(3 * k + 2));
        let centre = (first + last) / 2;
        pieces.push(Piece {
            a: (centre, AXIS - HELIX_RADIUS),
            b: (centre, AXIS + HELIX_RADIUS),
            half: (last - first) / 2 + LETTER_PITCH / 2 - 2,
            style: meaning_style(*meaning),
            owner: owners[k],
        });
        let cap_x = match meaning {
            Some(Meaning::Start) => Some(first - LETTER_PITCH / 2 - 6),
            Some(Meaning::Stop) => Some(last + LETTER_PITCH / 2 + 6),
            _ => None,
        };
        if let Some(x) = cap_x {
            caps.push(Piece {
                a: (x, AXIS - HELIX_RADIUS - 16),
                b: (x, AXIS + HELIX_RADIUS + 16),
                half: 4,
                style: STYLE_END_CAP,
                owner: owners[k],
            });
        }
        if let Some(m) = meaning {
            match label(
                &m.name().to_lowercase(),
                centre,
                AXIS + HELIX_RADIUS + 24,
                LABEL_SIZE,
                owners[k],
            ) {
                Verdict::Ok(p) => names.extend(p),
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
    }
    pieces.extend(caps);
    let pair = |i: usize| (sense[i], other[n - 1 - i]);
    for i in 0..n {
        let (s, p) = pair(i);
        if p != s.partner() {
            let (a, b, _) = at(i);
            pieces.push(Piece {
                a,
                b,
                half: 14,
                style: STYLE_GLOW,
                owner: owner_of(i),
            });
        }
    }
    for i in 1..n {
        let ((a0, b0, depth), (a1, b1, _)) = (at(i - 1), at(i));
        for (from, to, near) in [(a0, a1, depth), (b0, b1, -depth)] {
            pieces.push(Piece {
                a: from,
                b: to,
                half: 4 + near / 1000,
                style: STYLE_BACKBONE,
                owner: owner_of(i - 1),
            });
        }
    }
    for i in 0..n {
        let (a, b, depth) = at(i);
        let (s, p) = pair(i);
        let mid = (rung_x(i), AXIS);
        for (end, letter, near) in [(a, s, depth), (b, p, -depth)] {
            pieces.push(Piece {
                a: end,
                b: mid,
                half: 6 + 2 * near / 1000,
                style: STYLE_LETTER_A + u32::from(letter.bits()),
                owner: owner_of(i),
            });
        }
    }
    pieces.extend(names);
    let width = if n == 0 {
        2 * MARGIN
    } else {
        rung_x(n - 1) + LETTER_PITCH / 2 + MARGIN
    };
    let height = AXIS + HELIX_RADIUS + 24 + 20 + MARGIN;
    Verdict::Ok((pieces, (width, height)))
}
