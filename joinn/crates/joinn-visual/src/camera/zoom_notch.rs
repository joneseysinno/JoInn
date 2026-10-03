//! Wheel notches: 32 fine steps each, carrying into the level.

use joinn_frame::Verdict;

use super::{NOTCH, STEPS, Zoom};

impl Zoom {
    /// `notches` in (positive) or out (negative). A level outside the range is
    /// refused.
    pub fn notch(self, notches: i32) -> Verdict<Zoom> {
        let steps = i64::from(STEPS);
        let total = i64::from(self.level) * steps
            + i64::from(self.step)
            + i64::from(NOTCH) * i64::from(notches);
        let level = i32::try_from(total.div_euclid(steps)).unwrap_or(i32::MAX);
        let step = u32::try_from(total.rem_euclid(steps)).unwrap_or(0);
        Zoom::new(level, step)
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::Zoom;

    fn ok(v: Verdict<Zoom>) -> Zoom {
        match v {
            Verdict::Ok(z) => z,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn eight_notches_double_k() {
        let z = Zoom {
            level: -2,
            step: 95,
        };
        let (n0, d0) = z.k();
        let (n8, d8) = ok(z.notch(8)).k();
        assert_eq!(n8 * d0, 2 * n0 * d8);
        assert_eq!(
            ok(z.notch(8)),
            Zoom {
                level: -1,
                step: 95
            }
        );
    }

    #[test]
    fn one_notch_carries_into_the_level() {
        assert_eq!(
            ok(Zoom {
                level: 0,
                step: 224
            }
            .notch(1)),
            Zoom { level: 1, step: 0 }
        );
        assert_eq!(
            ok(Zoom { level: 1, step: 0 }.notch(-1)),
            Zoom {
                level: 0,
                step: 224
            }
        );
    }

    #[test]
    fn notching_past_either_end_is_refused_naming_the_level() {
        let Verdict::Refused(r) = (Zoom {
            level: 9,
            step: 224,
        })
        .notch(1) else {
            panic!("level 10 must be refused");
        };
        assert_eq!(
            r.reason,
            "zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9"
        );
        let Verdict::Refused(r) = (Zoom { level: -4, step: 0 }).notch(-1) else {
            panic!("level -5 must be refused");
        };
        assert_eq!(
            r.reason,
            "zoom: level -5 is outside −4 … 9; acceptance is a level from −4 to 9"
        );
    }
}
