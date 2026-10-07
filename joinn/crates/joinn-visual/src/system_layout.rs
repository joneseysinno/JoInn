//! A system's layout: its growing body, the response, and the lasso between
//! them (plan 7.4 §2.8). Every number is an integer in layout units.

mod check_lasso;
mod lasso_outline;
mod layout_system;
mod response_shape;
mod stroke_meets_rect;

pub use check_lasso::check_lasso;
pub use lasso_outline::lasso_outline;
pub use layout_system::layout_system;

use crate::layout::{CellBox, PortDot, Rect};

/// Cells per row of a growing body.
pub const ROW_CELLS: i64 = 6;
/// From the body's surface to the lasso's outline (stroke centre).
pub const LASSO_GAP: i64 = 2;
/// Each corner of the outline is cut by a straight stroke this long on each axis.
pub const LASSO_CORNER: i64 = 2;
/// From the outline's right side to the response's in-port side.
pub const NECK: i64 = 6;
/// Each arrowhead stroke reaches this far back on each axis.
pub const ARROW: i64 = 2;
/// The lasso's half-width, in sixteenths of a layout unit (½ unit).
pub const LASSO_HALF_WIDTH: i64 = 8;
/// From the system's origin to the body's surface.
pub const SYSTEM_PAD: i64 = 4;

/// One straight lasso stroke, centre line, in layout units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LassoStroke {
    /// Start.
    pub from: (i64, i64),
    /// End.
    pub to: (i64, i64),
}

/// A system's layout. `lasso` is the outline's eight strokes (clockwise from
/// the top), then the neck, then the arrowhead's two strokes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemLayout {
    /// The body's surface.
    pub surface: Rect,
    /// Grown cells, `numbers.0 …`, in growth order.
    pub cells: Vec<CellBox>,
    /// Waiting boxes, named for the instance each would grow.
    pub waiting: Vec<CellBox>,
    /// Each grown cell's and waiting box's in-port, then the response's out-port.
    pub ports: Vec<PortDot>,
    /// The response cell, outside the lasso.
    pub response: CellBox,
    /// The lasso's strokes.
    pub lasso: Vec<LassoStroke>,
    /// The arrowhead's tip.
    pub tip: (i64, i64),
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Cell, Contact, System, hash, parse_cell, parse_contact, parse_system};
    use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, Value, Verdict};
    use joinn_link::grow;
    use std::collections::BTreeMap;

    use super::{SystemLayout, check_lasso, lasso_outline, layout_system};

    fn int(n: i64) -> Value {
        match IntFrame::new().canonicalize(Term::int(n)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn cells() -> BTreeMap<Hash, Cell> {
        let mut out = BTreeMap::new();
        for src in [
            include_str!("../../../corpus/phase0/sum.cell"),
            include_str!("../../../corpus/phase0/cli_input.cell"),
            include_str!("../../../corpus/phase0/format.cell"),
        ] {
            match parse_cell(src, &FrameRegistry::phase1()) {
                Verdict::Ok(c) => {
                    out.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        out
    }

    fn system(src: &str) -> System {
        match parse_system(src) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn contacts() -> BTreeMap<Hash, Contact> {
        [
            include_str!("../../../corpus/phase74/counting.contact"),
            include_str!("../../../corpus/phase74/adding.contact"),
        ]
        .into_iter()
        .map(|src| match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => (hash(&c.coding), c),
            Verdict::Refused(r) => panic!("{}", r.reason),
        })
        .collect()
    }

    fn laid(s: &System, n: usize) -> SystemLayout {
        let (contacts, cells) = (contacts(), cells());
        let grown = match grow(s, &contacts, &cells, &vec![int(1); n]) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let waiting = s.regulatory.waiting.values().copied().next().unwrap_or(0);
        match layout_system(s, &contacts, &cells, &grown, waiting) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn the_four_rules_hold_at_every_size_through_both_row_wraps() {
        let counting = system(include_str!("../../../corpus/phase74/counting.system"));
        let adding = system(include_str!("../../../corpus/phase74/adding.system"));
        for (s, waiting) in [(&counting, 1usize), (&adding, 2)] {
            for n in 0..=13 {
                let l = laid(s, n);
                assert_eq!((l.cells.len(), l.waiting.len()), (n, waiting), "size {n}");
                if let Verdict::Refused(r) = check_lasso(&l) {
                    panic!("size {n}: {}", r.reason);
                }
            }
        }
        let heights: Vec<i64> = [5, 6, 11, 12]
            .iter()
            .map(|n| laid(&counting, *n).surface.h)
            .collect();
        assert_eq!(heights, vec![14, 24, 24, 34], "rows wrap at 7 and 13 boxes");
        let l = laid(&counting, 0);
        assert_eq!(
            (l.surface, l.response.rect, l.tip),
            (
                crate::layout::Rect {
                    x: 4,
                    y: 4,
                    w: 20,
                    h: 14
                },
                crate::layout::Rect {
                    x: 32,
                    y: 6,
                    w: 12,
                    h: 10
                },
                (32, 11)
            )
        );
    }

    #[test]
    fn a_stroke_moved_inward_is_refused_by_rule_1() {
        let counting = system(include_str!("../../../corpus/phase74/counting.system"));
        let mut l = laid(&counting, 3);
        l.lasso[0].from.1 += 2;
        l.lasso[0].to.1 += 2;
        match check_lasso(&l) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "lasso: stroke 0 meets the surface; acceptance is a lasso clear of every cell and the surface (rule 1)"
            ),
            Verdict::Ok(()) => panic!("a stroke on the surface passed"),
        }
        let mut whole = laid(&counting, 3);
        let inward = lasso_outline(&whole.surface, 0);
        whole.lasso.splice(..8, inward);
        assert!(matches!(check_lasso(&whole), Verdict::Refused(_)));
    }
}
