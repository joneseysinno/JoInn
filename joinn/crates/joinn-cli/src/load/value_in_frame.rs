//! Canonicalize a host line in the port's frame.

use joinn_frame::{Frame, FrameId, FrameRef, IntFrame, Term, Value, Verdict};

/// Canonicalize `line` in `frame`. Text stays text. ℤ is a decimal integer.
pub(crate) fn value_in_frame(frame: &FrameRef, line: &str) -> Result<Value, String> {
    let term = match frame.id {
        FrameId::Text => return super::text_value::text_value(line),
        FrameId::Int => {
            let n = line
                .parse::<i64>()
                .map_err(|_| format!("\"{line}\" is not in ℤ; acceptance is a decimal integer"))?;
            Term::int(n)
        }
        FrameId::Rat => {
            return Err(format!(
                "\"{line}\" is not read as ℚ; acceptance is a host that admits that frame"
            ));
        }
    };
    match IntFrame::new().canonicalize(term) {
        Verdict::Ok(v) => Ok(v),
        Verdict::Refused(r) => Err(r.reason),
    }
}
