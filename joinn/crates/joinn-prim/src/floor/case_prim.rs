//! `case(v) → (tag, parts)`.

use joinn_frame::{Case, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::canon_int::canon_int;
use super::frame_of::frame_of;
use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// `case(v) → (tag, parts)`. Tag 0 is the generator; `k ≥ 1` is the k-th constructor.
pub struct CasePrim;
impl Oracle for CasePrim {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("case missing argument"));
        };
        let frame = frame_of(v);
        let ctors = frame.constructors();
        match frame.case(v) {
            Case::Generator => canon_int(0).map(|tag| BTreeMap::from([(0, tag)])),
            Case::Built { op, parts } => {
                let tag = match ctors.iter().position(|c| c.as_str() == op.as_str()) {
                    Some(i) => i as i64,
                    None => {
                        return Verdict::Refused(refuse(&format!(
                            "case constructor {} is not in constructors()",
                            op.as_str()
                        )));
                    }
                };
                let tag_v = match canon_int(tag) {
                    Verdict::Ok(t) => t,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                };
                let mut out = BTreeMap::from([(0, tag_v)]);
                if let Some(p0) = parts.first() {
                    out.insert(1, p0.clone());
                }
                if let Some(p1) = parts.get(1) {
                    out.insert(2, p1.clone());
                }
                Verdict::Ok(out)
            }
        }
    }
}
impl Reference for CasePrim {
    fn name(&self) -> &'static str {
        "case"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("build")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
