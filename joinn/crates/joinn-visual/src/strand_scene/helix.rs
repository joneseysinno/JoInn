//! Where a letter's pair sits on the helix.

use super::{AXIS, CODON_GAP, HELIX_RADIUS, LETTER_PITCH, MARGIN, SINES};

/// Letter `i`'s point on the strand, its partner's point, and its depth in
/// thousandths (toward the eye on the strand's side when positive). The pair
/// turns 30° a letter; the view looks a quarter radius down onto the axis, so
/// no rung shrinks to a point.
pub(super) fn helix(i: i64) -> ((i64, i64), (i64, i64), i64) {
    let x = MARGIN + LETTER_PITCH / 2 + i * LETTER_PITCH + i.div_euclid(3) * CODON_GAP;
    let j = i.rem_euclid(12) as usize;
    let (s, c) = (SINES[j], SINES[(j + 3) % 12]);
    let dx = HELIX_RADIUS * c / 4000;
    let dy = HELIX_RADIUS * (4 * s + c) / 4000;
    ((x + dx, AXIS + dy), (x - dx, AXIS - dy), c)
}

#[cfg(test)]
mod tests {
    use super::helix;
    use crate::strand_scene::{AXIS, CODON_GAP, LETTER_PITCH, MARGIN};

    #[test]
    fn the_two_sides_mirror_through_the_axis_and_never_meet() {
        for i in 0..24 {
            let (a, b, _) = helix(i);
            let rung = MARGIN + LETTER_PITCH / 2 + i * LETTER_PITCH + (i / 3) * CODON_GAP;
            assert_eq!(a.0 + b.0, 2 * rung, "letter {i}");
            assert_eq!(a.1 + b.1, 2 * AXIS, "letter {i}");
            assert_ne!(a, b, "letter {i}");
        }
        assert_eq!(helix(0).2, 1000);
        assert_eq!(helix(6).2, -1000);
    }
}
