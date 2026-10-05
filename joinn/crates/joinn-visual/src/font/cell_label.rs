//! A cell's label: its instance name at its top-left.

use joinn_frame::Verdict;

use super::{LABEL_SIZE, TextStroke, text_strokes};
use crate::layout::CellBox;

/// The instance name, cap height 1 layout unit, with its grid origin at
/// `(cell.x + 1, cell.y + 3/4)` in the body chart.
pub fn cell_label(cell: &CellBox) -> Verdict<Vec<TextStroke>> {
    let origin = (16 * (cell.rect.x + 1), 16 * cell.rect.y + 12);
    text_strokes(&cell.instance, origin, LABEL_SIZE)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::cell_label;
    use crate::fixtures::calculator;
    use crate::layout::layout;

    #[test]
    fn the_s_of_sum_occupies_the_plan_s_box_exactly() {
        let (body, cells) = calculator();
        let Verdict::Ok(l) = layout(&body, &cells) else {
            panic!("the calculator lays out");
        };
        let Some(sum) = l.cells.iter().find(|c| c.instance == "sum") else {
            panic!("the calculator has sum");
        };
        assert_eq!((sum.rect.x, sum.rect.y), (24, 4));
        let Verdict::Ok(strokes) = cell_label(sum) else {
            panic!("sum has glyphs");
        };
        let s: Vec<_> = strokes.iter().take(11).collect();
        let xs: Vec<i64> = s.iter().flat_map(|t| [t.x0, t.x1]).collect();
        let ys: Vec<i64> = s.iter().flat_map(|t| [t.y0, t.y1]).collect();
        assert_eq!(
            (xs.iter().min(), xs.iter().max()),
            (Some(&400), Some(&412)),
            "x 25 … 25 3/4"
        );
        assert_eq!(
            (ys.iter().min(), ys.iter().max()),
            (Some(&76), Some(&92)),
            "y 4 3/4 … 5 3/4"
        );
        assert!(s.iter().all(|t| t.half_width == 1));
        let xs_rest: Vec<i64> = strokes.iter().skip(11).flat_map(|t| [t.x0, t.x1]).collect();
        assert!(xs_rest.iter().all(|&x| x >= 416), "u and m follow the s");
    }
}
