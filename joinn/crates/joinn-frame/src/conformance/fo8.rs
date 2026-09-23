//! `fo8`.

use crate::frame::{Case, Frame, OpName};
use crate::verdict::Verdict;

pub(in crate::conformance) fn fo8(
    frame: &dyn Frame,
    seed: u64,
    n: u32,
    failures: &mut Vec<String>,
) {
    let generators: Vec<OpName> = frame
        .signature()
        .iter()
        .filter(|(_, arity)| *arity == 0)
        .map(|(op, _)| op.clone())
        .collect();
    for i in 0..n {
        let v = frame.generate(seed.wrapping_add(u64::from(i) + 409), 16);
        if !super::fo8_one(frame, &v, &generators, seed, failures) {
            return;
        }
        for s in frame.shrink(&v) {
            if !super::fo8_one(frame, &s, &generators, seed, failures) {
                return;
            }
        }
    }
    for op in &generators {
        match frame.apply_op(op, &[]) {
            Verdict::Ok(g) => match frame.case(&g) {
                Case::Generator => {}
                Case::Built { .. } => {
                    failures.push(format!(
                        "FO8 case(apply_op({})) is not Generator (seed {seed})",
                        op.as_str()
                    ));
                    return;
                }
            },
            Verdict::Refused(_) => {
                failures.push(format!(
                    "FO8 generator {} refused with no args (seed {seed})",
                    op.as_str()
                ));
                return;
            }
        }
    }
}
