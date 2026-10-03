//! The zoom of a whole `k` from 1 to 511 (Phase 6's cameras).

use joinn_frame::Verdict;

use super::Zoom;
use crate::refuse::refuse;

impl Zoom {
    /// Every whole `k` from 1 to 511 is exactly one `(level, step)`.
    pub fn from_whole_k(k: i64) -> Verdict<Zoom> {
        if !(1..=511).contains(&k) {
            return refuse(format!(
                "zoom: k {k} has no level and step; acceptance is a whole k from 1 to 511"
            ));
        }
        let level = 63 - k.leading_zeros();
        let step = (k << (8 - level)) - 256;
        Zoom::new(
            i32::try_from(level).unwrap_or(i32::MAX),
            u32::try_from(step).unwrap_or(u32::MAX),
        )
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::Zoom;

    #[test]
    fn every_whole_k_from_1_to_511_converts_and_back() {
        for k in 1..=511 {
            let z = match Zoom::from_whole_k(k) {
                Verdict::Ok(z) => z,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(z.whole_k(), Some(k), "{z:?}");
            assert_eq!(z.k(), (k, 1));
        }
        assert_eq!(
            Zoom::from_whole_k(40),
            Verdict::Ok(Zoom { level: 5, step: 64 })
        );
    }

    #[test]
    fn k_0_and_512_are_refused() {
        for k in [0, 512] {
            let Verdict::Refused(r) = Zoom::from_whole_k(k) else {
                panic!("k {k} must be refused");
            };
            assert_eq!(
                r.reason,
                format!("zoom: k {k} has no level and step; acceptance is a whole k from 1 to 511")
            );
        }
    }
}
