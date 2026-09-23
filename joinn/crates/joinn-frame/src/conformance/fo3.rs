//! `fo3`.

use crate::frame::Frame;
use crate::verdict::Verdict;

pub(in crate::conformance) fn fo3(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 31), 16);
        match frame.canonicalize(v.term().clone()) {
            Verdict::Ok(once) => match frame.canonicalize(once.term().clone()) {
                Verdict::Ok(twice) if frame.eq(&once, &twice) => {}
                _ => {
                    failures.push(format!("FO3 canonicalize is not idempotent (seed {seed})"));
                    return;
                }
            },
            Verdict::Refused(_) => {
                failures.push(format!(
                    "FO3 canonicalize refused a generated value (seed {seed})"
                ));
                return;
            }
        }
    }
}
