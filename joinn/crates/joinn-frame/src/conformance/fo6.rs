//! `fo6`.

use crate::frame::{Frame, FrameRef};
use crate::term::Term;
use crate::verdict::Verdict;

pub(in crate::conformance) fn fo6(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    let foreign = Term::Seq(vec![Term::text("adversarial"), Term::int(0)]);
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 89), 16);
        for (op, arity) in frame.signature().iter() {
            let mut args = Vec::new();
            for _ in 0..arity {
                args.push(v.clone());
            }
            let _ = frame.apply_op(op, &args);
            if arity > 0 {
                if let Verdict::Ok(bad) = frame.canonicalize(foreign.clone()) {
                    args[0] = bad;
                    match frame.apply_op(op, &args) {
                        Verdict::Refused(_) => {}
                        Verdict::Ok(_) => {
                            // A frame may accept its own seq shape; only refuse true foreigners.
                        }
                    }
                } else {
                    // Build a value in a different frame by going through the other constructors.
                    let other = if op.as_str() == "chr" {
                        crate::frames::RatFrame::new().generate(1, 4)
                    } else if frame.reference() == FrameRef::text() {
                        crate::frames::IntFrame::new().generate(1, 4)
                    } else {
                        crate::frames::TextFrame::new().generate(1, 4)
                    };
                    args[0] = other;
                    match frame.apply_op(op, &args) {
                        Verdict::Refused(_) => {}
                        Verdict::Ok(_) => {
                            failures.push(format!(
                                "FO6 op {} accepted an out-of-frame argument (seed {seed})",
                                op.as_str()
                            ));
                            return;
                        }
                    }
                }
            }
        }
    }
}
