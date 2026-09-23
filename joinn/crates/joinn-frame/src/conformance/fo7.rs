//! `fo7`.

use crate::frame::{Frame, FrameRef};
use crate::verdict::Verdict;

pub(in crate::conformance) fn fo7(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    let int = crate::frames::IntFrame::new();
    for ext in frame.extensions() {
        if *ext != FrameRef::int() {
            failures.push(format!("FO7 unhandled extension {ext}"));
            return;
        }
        for i in 0..n {
            let x = int.generate(seed.wrapping_add(u64::from(i) + 101), 16);
            let embedded = match frame.embed(ext, &x) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => {
                    failures.push(format!("FO7 embed refused a ℤ value (seed {seed})"));
                    return;
                }
            };
            match frame.restrict(ext, &embedded) {
                Verdict::Ok(back) if int.eq(&back, &x) => {}
                _ => {
                    failures.push(format!("FO7 ρ(ι(x)) ≠ x (seed {seed})"));
                    return;
                }
            }
            let y = int.generate(seed.wrapping_add(u64::from(i) + 202), 16);
            let ey = match frame.embed(ext, &y) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => {
                    failures.push("FO7 embed refused a second ℤ value".into());
                    return;
                }
            };
            if int.eq(&x, &y) != frame.eq(&embedded, &ey) {
                failures.push(format!("FO7 ι does not preserve eq (seed {seed})"));
                return;
            }
            for (op, arity) in int.signature().iter() {
                if !super::preserves_op(frame, &int, ext, op, arity, seed, i) {
                    failures.push(format!(
                        "FO7 ι does not preserve {} (seed {seed})",
                        op.as_str()
                    ));
                    return;
                }
            }
        }
    }
}
