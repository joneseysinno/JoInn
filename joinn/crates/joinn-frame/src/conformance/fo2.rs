//! `fo2`.

use crate::frame::Frame;
use crate::verdict::Verdict;

pub(in crate::conformance) fn fo2(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 17), 16);
        let printed = frame.print(&v);
        match frame.parse(&printed) {
            Verdict::Ok(back) if frame.eq(&back, &v) => {}
            _ => {
                failures.push(format!(
                    "FO2 parse(print(v)) ≠ v (seed {seed}, printed {printed})"
                ));
                return;
            }
        }
    }
}
