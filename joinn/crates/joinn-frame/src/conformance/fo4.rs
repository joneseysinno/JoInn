//! `fo4`.

use crate::frame::Frame;

pub(in crate::conformance) fn fo4(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 47), 16);
        if !frame.contains(v.term()) {
            failures.push(format!(
                "FO4 generated value is not contained (seed {seed})"
            ));
            return;
        }
    }
}
