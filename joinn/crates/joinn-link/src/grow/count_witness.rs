//! The response computed by counting: the witness, not the engine.

use joinn_frame::{Frame, FrameRef, IntFrame, OpName, Value, Verdict};

const MAX_STEPS: u64 = 1_000_000;

/// Start at `zero`; for each input `v` take `|v|` steps of `succ` (or `pred`
/// when `v < 0`). No sum cell, no `+` on values, no register: only the next
/// and previous integer.
pub fn count_witness(inputs: &[Value]) -> Verdict<Value> {
    let frame = IntFrame::new();
    let mut count = match frame.apply_op(&OpName("zero".into()), &[]) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for input in inputs {
        if input.frame() != &FrameRef::int() {
            return crate::refuse(format!(
                "witness: {} is not in ℤ 1; acceptance is a value in ℤ 1",
                input.print_literal()
            ));
        }
        let steps = input
            .print_term()
            .parse::<i64>()
            .ok()
            .map(|v| (v.unsigned_abs(), v < 0))
            .filter(|(n, _)| *n <= MAX_STEPS);
        let Some((steps, down)) = steps else {
            return crate::refuse("witness: too many steps; acceptance is |v| ≤ 1000000");
        };
        let op = OpName(if down { "pred" } else { "succ" }.into());
        for _ in 0..steps {
            count = match frame.apply_op(&op, std::slice::from_ref(&count)) {
                Verdict::Ok(v) => v,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
        }
    }
    Verdict::Ok(count)
}
