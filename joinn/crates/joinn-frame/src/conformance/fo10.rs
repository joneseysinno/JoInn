//! `fo10`.

use crate::frame::Frame;

pub(in crate::conformance) fn fo10(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    let ctors = frame.constructors();
    let mut saw_built = false;
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 631), 16);
        if !super::fo10_one(frame, &v, ctors, seed, failures, &mut saw_built) {
            return;
        }
        for s in frame.shrink(&v) {
            if !super::fo10_one(frame, &s, ctors, seed, failures, &mut saw_built) {
                return;
            }
        }
    }
    if saw_built && ctors.is_empty() {
        failures.push(format!(
            "FO10 constructors() is empty but case reported Built (seed {seed})"
        ));
    }
}
