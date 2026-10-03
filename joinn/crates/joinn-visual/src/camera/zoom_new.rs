//! A zoom from its level and step, refused outside the ranges.

use joinn_frame::Verdict;

use super::{LEVEL_MAX, LEVEL_MIN, STEPS, Zoom};
use crate::refuse::refuse;

impl Zoom {
    /// `level` in `LEVEL_MIN ..= LEVEL_MAX`, `step` in `0 .. STEPS`.
    pub fn new(level: i32, step: u32) -> Verdict<Zoom> {
        if !(LEVEL_MIN..=LEVEL_MAX).contains(&level) {
            return refuse(format!(
                "zoom: level {level} is outside −4 … 9; acceptance is a level from −4 to 9"
            ));
        }
        if step >= STEPS {
            return refuse(format!(
                "zoom: step {step} is outside 0 … 255; acceptance is a step from 0 to 255"
            ));
        }
        Verdict::Ok(Zoom { level, step })
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::Zoom;

    fn reason(v: Verdict<Zoom>) -> String {
        match v {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(z) => panic!("{z:?} must be refused"),
        }
    }

    #[test]
    fn levels_outside_the_range_are_refused_with_the_plan_wording() {
        assert_eq!(
            reason(Zoom::new(-5, 0)),
            "zoom: level -5 is outside −4 … 9; acceptance is a level from −4 to 9"
        );
        assert_eq!(
            reason(Zoom::new(10, 0)),
            "zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9"
        );
        assert_eq!(Zoom::new(-4, 0), Verdict::Ok(Zoom { level: -4, step: 0 }));
        assert_eq!(
            Zoom::new(9, 255),
            Verdict::Ok(Zoom {
                level: 9,
                step: 255
            })
        );
    }

    #[test]
    fn a_step_of_256_is_refused() {
        assert_eq!(
            reason(Zoom::new(0, 256)),
            "zoom: step 256 is outside 0 … 255; acceptance is a step from 0 to 255"
        );
    }
}
