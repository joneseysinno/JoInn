//! An out-port's value, right-aligned beside the port.

use joinn_frame::Verdict;

use super::{ADVANCE, GRID_H, GRID_W, TextStroke, VALUE_SIZE, text_strokes, value_text};
use crate::layout::PortDot;

/// The value as `value_text` prints it, cap height 2, right-aligned so its
/// last glyph ends 2 layout units left of the port centre, cap centred on the
/// port's y.
pub fn port_value(port: &PortDot, value: &str) -> Verdict<Vec<TextStroke>> {
    let text = value_text(value);
    let glyphs = i64::try_from(text.chars().count()).unwrap_or(0);
    let width = ((glyphs - 1).max(0) * ADVANCE + GRID_W) * VALUE_SIZE.unit;
    let origin = (
        16 * (port.x - 2) - width,
        16 * port.y - GRID_H * VALUE_SIZE.unit / 2,
    );
    text_strokes(&text, origin, VALUE_SIZE)
}

#[cfg(test)]
mod tests {
    use joinn_dna::Direction;
    use joinn_frame::Verdict;
    use joinn_link::Address;

    use super::port_value;
    use crate::layout::PortDot;

    fn port() -> PortDot {
        PortDot {
            address: Address {
                instance: "sum".to_owned(),
                port: 2,
            },
            direction: Direction::Out,
            x: 36,
            y: 7,
        }
    }

    #[test]
    fn a_value_ends_two_units_left_of_the_port_with_its_cap_centred() {
        let Verdict::Ok(strokes) = port_value(&port(), "02345") else {
            panic!("digits have glyphs");
        };
        let xs: Vec<i64> = strokes.iter().flat_map(|t| [t.x0, t.x1]).collect();
        let ys: Vec<i64> = strokes.iter().flat_map(|t| [t.y0, t.y1]).collect();
        assert_eq!(xs.iter().max(), Some(&(16 * 34)));
        assert_eq!(xs.iter().min(), Some(&(16 * 34 - 120)));
        assert_eq!(
            (ys.iter().min(), ys.iter().max()),
            (Some(&(16 * 6)), Some(&(16 * 8)))
        );
        assert!(strokes.iter().all(|t| t.half_width == 2));
    }
}
