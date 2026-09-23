//! `fo1`.

use crate::frame::Frame;

pub(in crate::conformance) fn fo1(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let a = frame.generate(seed.wrapping_add(u64::from(i)), 16);
        let b = frame.generate(seed.wrapping_add(u64::from(i).wrapping_mul(3) + 1), 16);
        let eq = frame.eq(&a, &b);
        let ident = a == b;
        if eq != ident {
            failures.push(format!(
                "FO1 equality does not coincide with canonical identity (seed {seed})"
            ));
            return;
        }
    }
}
