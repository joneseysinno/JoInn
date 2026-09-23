//! `fo9`.

use crate::frame::Frame;

pub(in crate::conformance) fn fo9(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 521), 16);
        if !super::fo9_ok(frame, &v) {
            failures.push(format!("FO9 case is not well-founded (seed {seed})"));
            return;
        }
        for s in frame.shrink(&v) {
            if !super::fo9_ok(frame, &s) {
                failures.push(format!(
                    "FO9 case is not well-founded on a shrunk value (seed {seed})"
                ));
                return;
            }
        }
    }
}
