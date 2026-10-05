//! The glyph set: each character's strokes, written as polylines on the grid.

use joinn_frame::Verdict;

use super::GlyphStroke;
use crate::refuse::refuse;

type Polyline = &'static [(i64, i64)];

/// Every character with a glyph, and its polylines. Shapes are presentation.
const GLYPHS: &[(char, &[Polyline])] = &[
    (' ', &[]),
    (
        '0',
        &[
            &[
                (1, 0),
                (5, 0),
                (6, 1),
                (6, 7),
                (5, 8),
                (1, 8),
                (0, 7),
                (0, 1),
                (1, 0),
            ],
            &[(5, 1), (1, 7)],
        ],
    ),
    ('1', &[&[(1, 2), (3, 0), (3, 8)], &[(1, 8), (5, 8)]]),
    (
        '2',
        &[&[(0, 1), (1, 0), (5, 0), (6, 1), (6, 3), (0, 8), (6, 8)]],
    ),
    (
        '3',
        &[&[
            (0, 0),
            (6, 0),
            (3, 3),
            (5, 3),
            (6, 4),
            (6, 7),
            (5, 8),
            (1, 8),
            (0, 7),
        ]],
    ),
    ('4', &[&[(4, 8), (4, 0), (0, 5), (6, 5)]]),
    (
        '5',
        &[&[
            (6, 0),
            (0, 0),
            (0, 3),
            (5, 3),
            (6, 4),
            (6, 7),
            (5, 8),
            (0, 8),
        ]],
    ),
    (
        '6',
        &[&[
            (5, 0),
            (1, 0),
            (0, 1),
            (0, 7),
            (1, 8),
            (5, 8),
            (6, 7),
            (6, 5),
            (5, 4),
            (0, 4),
        ]],
    ),
    ('7', &[&[(0, 0), (6, 0), (2, 8)]]),
    (
        '8',
        &[
            &[
                (1, 0),
                (5, 0),
                (6, 1),
                (6, 3),
                (5, 4),
                (1, 4),
                (0, 5),
                (0, 7),
                (1, 8),
                (5, 8),
                (6, 7),
                (6, 5),
                (5, 4),
            ],
            &[(1, 4), (0, 3), (0, 1), (1, 0)],
        ],
    ),
    (
        '9',
        &[&[
            (6, 4),
            (1, 4),
            (0, 3),
            (0, 1),
            (1, 0),
            (5, 0),
            (6, 1),
            (6, 7),
            (5, 8),
            (1, 8),
        ]],
    ),
    ('a', &[&[(0, 8), (3, 0), (6, 8)], &[(1, 5), (5, 5)]]),
    (
        'b',
        &[
            &[(0, 0), (0, 8), (5, 8), (6, 7), (6, 5), (5, 4), (0, 4)],
            &[(0, 0), (4, 0), (5, 1), (5, 3), (4, 4)],
        ],
    ),
    (
        'c',
        &[&[
            (6, 1),
            (5, 0),
            (1, 0),
            (0, 1),
            (0, 7),
            (1, 8),
            (5, 8),
            (6, 7),
        ]],
    ),
    (
        'd',
        &[&[(0, 0), (0, 8), (4, 8), (6, 6), (6, 2), (4, 0), (0, 0)]],
    ),
    ('e', &[&[(6, 0), (0, 0), (0, 8), (6, 8)], &[(0, 4), (4, 4)]]),
    ('f', &[&[(6, 0), (0, 0), (0, 8)], &[(0, 4), (4, 4)]]),
    (
        'g',
        &[&[
            (6, 1),
            (5, 0),
            (1, 0),
            (0, 1),
            (0, 7),
            (1, 8),
            (5, 8),
            (6, 7),
            (6, 4),
            (3, 4),
        ]],
    ),
    (
        'h',
        &[&[(0, 0), (0, 8)], &[(6, 0), (6, 8)], &[(0, 4), (6, 4)]],
    ),
    (
        'i',
        &[&[(1, 0), (5, 0)], &[(3, 0), (3, 8)], &[(1, 8), (5, 8)]],
    ),
    (
        'j',
        &[&[(2, 0), (6, 0)], &[(5, 0), (5, 7), (4, 8), (1, 8), (0, 7)]],
    ),
    (
        'k',
        &[&[(0, 0), (0, 8)], &[(6, 0), (0, 6)], &[(2, 4), (6, 8)]],
    ),
    ('l', &[&[(0, 0), (0, 8), (6, 8)]]),
    ('m', &[&[(0, 8), (0, 0), (3, 4), (6, 0), (6, 8)]]),
    ('n', &[&[(0, 8), (0, 0), (6, 8), (6, 0)]]),
    (
        'o',
        &[&[
            (1, 0),
            (5, 0),
            (6, 1),
            (6, 7),
            (5, 8),
            (1, 8),
            (0, 7),
            (0, 1),
            (1, 0),
        ]],
    ),
    (
        'p',
        &[&[(0, 8), (0, 0), (5, 0), (6, 1), (6, 3), (5, 4), (0, 4)]],
    ),
    (
        'q',
        &[
            &[
                (1, 0),
                (5, 0),
                (6, 1),
                (6, 7),
                (5, 8),
                (1, 8),
                (0, 7),
                (0, 1),
                (1, 0),
            ],
            &[(4, 6), (6, 8)],
        ],
    ),
    (
        'r',
        &[
            &[(0, 8), (0, 0), (5, 0), (6, 1), (6, 3), (5, 4), (0, 4)],
            &[(3, 4), (6, 8)],
        ],
    ),
    (
        's',
        &[&[
            (6, 1),
            (5, 0),
            (1, 0),
            (0, 1),
            (0, 3),
            (1, 4),
            (5, 4),
            (6, 5),
            (6, 7),
            (5, 8),
            (1, 8),
            (0, 7),
        ]],
    ),
    ('t', &[&[(0, 0), (6, 0)], &[(3, 0), (3, 8)]]),
    ('u', &[&[(0, 0), (0, 7), (1, 8), (5, 8), (6, 7), (6, 0)]]),
    ('v', &[&[(0, 0), (3, 8), (6, 0)]]),
    ('w', &[&[(0, 0), (1, 8), (3, 4), (5, 8), (6, 0)]]),
    ('x', &[&[(0, 0), (6, 8)], &[(6, 0), (0, 8)]]),
    ('y', &[&[(0, 0), (3, 4), (6, 0)], &[(3, 4), (3, 8)]]),
    ('z', &[&[(0, 0), (6, 0), (0, 8), (6, 8)]]),
    ('_', &[&[(0, 9), (6, 9)]]),
    ('.', &[&[(3, 7), (3, 8)]]),
    (',', &[&[(3, 7), (2, 9)]]),
    ('-', &[&[(1, 4), (5, 4)]]),
    ('+', &[&[(1, 4), (5, 4)], &[(3, 2), (3, 6)]]),
    ('=', &[&[(1, 3), (5, 3)], &[(1, 5), (5, 5)]]),
    ('×', &[&[(1, 2), (5, 6)], &[(5, 2), (1, 6)]]),
    (
        '÷',
        &[&[(1, 4), (5, 4)], &[(3, 1), (3, 2)], &[(3, 6), (3, 7)]],
    ),
    ('(', &[&[(4, 0), (2, 2), (2, 6), (4, 8)]]),
    (')', &[&[(2, 0), (4, 2), (4, 6), (2, 8)]]),
    (
        '…',
        &[&[(0, 7), (0, 8)], &[(3, 7), (3, 8)], &[(6, 7), (6, 8)]],
    ),
];

/// Every character with a glyph, in table order.
pub const STROKE_SET: &str = " 0123456789abcdefghijklmnopqrstuvwxyz_.,-+=×÷()…";

/// The strokes of `c`, one per polyline segment, in polyline order.
pub fn glyph(c: char) -> Verdict<Vec<GlyphStroke>> {
    let Some((_, lines)) = GLYPHS.iter().find(|(g, _)| *g == c) else {
        return refuse(format!(
            "text: '{c}' has no glyph; acceptance is a character from the stroke set"
        ));
    };
    Verdict::Ok(
        lines
            .iter()
            .flat_map(|line| line.windows(2))
            .filter_map(|pair| match pair {
                [(x0, y0), (x1, y1)] => Some(GlyphStroke {
                    x0: *x0,
                    y0: *y0,
                    x1: *x1,
                    y1: *y1,
                }),
                _ => None,
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use joinn_frame::Verdict;

    use super::{GLYPHS, STROKE_SET, glyph};
    use crate::font::{GRID_DESCENT, GRID_W, GlyphStroke};

    fn strokes(c: char) -> Vec<GlyphStroke> {
        match glyph(c) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn the_stroke_set_is_the_table_and_the_plan_s_characters() {
        let table: String = GLYPHS.iter().map(|(c, _)| *c).collect();
        assert_eq!(table, STROKE_SET);
        for c in ('0'..='9').chain('a'..='z').chain("_.,-+=×÷()… ".chars()) {
            assert!(STROKE_SET.contains(c), "{c} is in the plan's set");
        }
        assert_eq!(STROKE_SET.chars().count(), 48);
    }

    #[test]
    fn every_stroke_lies_inside_the_6_by_10_box() {
        for c in STROKE_SET.chars() {
            for s in strokes(c) {
                for (x, y) in [(s.x0, s.y0), (s.x1, s.y1)] {
                    assert!(
                        (0..=GRID_W).contains(&x) && (0..=GRID_DESCENT).contains(&y),
                        "'{c}' has a stroke point ({x}, {y}) outside 6×10"
                    );
                }
            }
        }
    }

    #[test]
    fn no_two_characters_have_the_same_stroke_set() {
        type Segment = ((i64, i64), (i64, i64));
        let mut seen: BTreeMap<BTreeSet<Segment>, char> = BTreeMap::new();
        for c in STROKE_SET.chars() {
            let set: BTreeSet<_> = strokes(c)
                .into_iter()
                .map(|s| {
                    let (a, b) = ((s.x0, s.y0), (s.x1, s.y1));
                    if a <= b { (a, b) } else { (b, a) }
                })
                .collect();
            if let Some(other) = seen.insert(set, c) {
                panic!("'{c}' and '{other}' have the same stroke set");
            }
        }
        assert_eq!(seen.len(), 48);
    }

    #[test]
    fn a_character_outside_the_set_is_refused_naming_it() {
        for c in ['S', '!', 'é'] {
            match glyph(c) {
                Verdict::Refused(r) => assert_eq!(
                    r.reason,
                    format!(
                        "text: '{c}' has no glyph; acceptance is a character from the stroke set"
                    )
                ),
                Verdict::Ok(_) => panic!("'{c}' has no glyph"),
            }
        }
    }

    #[test]
    fn s_spans_the_whole_6_by_8_box() {
        let s = strokes('s');
        let xs: Vec<i64> = s.iter().flat_map(|t| [t.x0, t.x1]).collect();
        let ys: Vec<i64> = s.iter().flat_map(|t| [t.y0, t.y1]).collect();
        assert_eq!(xs.iter().min(), Some(&0));
        assert_eq!(xs.iter().max(), Some(&6));
        assert_eq!(ys.iter().min(), Some(&0));
        assert_eq!(ys.iter().max(), Some(&8));
        assert_eq!(s.len(), 11);
    }
}
