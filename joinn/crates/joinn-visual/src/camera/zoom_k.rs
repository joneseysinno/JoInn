//! `k`, exactly, as a fraction in lowest terms.

use super::Zoom;

impl Zoom {
    /// `k = 2^level · (256 + step) / 256` as `(numerator, denominator)` in
    /// lowest terms. The denominator is a power of two.
    pub fn k(self) -> (i64, i64) {
        let base = 256 + i64::from(self.step);
        let (num, den) = if self.level >= 0 {
            (base << self.level, 256)
        } else {
            (base, 256i64 << -self.level)
        };
        let shift = num.trailing_zeros().min(den.trailing_zeros());
        (num >> shift, den >> shift)
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::{LEVEL_MAX, LEVEL_MIN, STEPS, Zoom};

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    #[test]
    fn k_is_2_to_the_level_times_256_plus_step_over_256_at_every_zoom() {
        let mut checked = 0;
        for level in LEVEL_MIN..=LEVEL_MAX {
            for step in 0..STEPS {
                let (num, den) = Zoom { level, step }.k();
                let base = 256 + i64::from(step);
                let (want_num, want_den) = if level >= 0 {
                    (base * 2i64.pow(level as u32), 256)
                } else {
                    (base, 256 * 2i64.pow((-level) as u32))
                };
                assert_eq!(num * want_den, den * want_num, "level {level} step {step}");
                assert_eq!(gcd(num, den), 1, "level {level} step {step}");
                checked += 1;
            }
        }
        assert_eq!(checked, 14 * 256);
    }

    #[test]
    fn the_frame_zoom_of_the_grove_is_351_over_1024() {
        assert_eq!(
            Zoom {
                level: -2,
                step: 95
            }
            .k(),
            (351, 1024)
        );
        assert_eq!(Zoom { level: -4, step: 0 }.k(), (1, 16));
        assert_eq!(Zoom { level: 9, step: 0 }.k(), (512, 1));
    }
}
