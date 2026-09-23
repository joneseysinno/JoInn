//! `fo5`.

use crate::frame::Frame;

pub(in crate::conformance) fn fo5(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 61), 16);
        if !super::shrink_ok(frame, &v) {
            failures.push(format!(
                "FO5 shrink grew, cycled, or failed to terminate (seed {seed})"
            ));
            return;
        }
    }
}
