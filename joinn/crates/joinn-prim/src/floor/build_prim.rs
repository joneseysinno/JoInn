//! `build(witness, tag, parts)`: the frame's tag-th declared constructor.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::frame_for_build::frame_for_build;
use super::int_tag::int_tag;
use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// `build(witness, tag, parts)`: the frame's tag-th declared constructor.
pub struct BuildPrim;
impl Oracle for BuildPrim {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(witness) = inputs.get(&0) else {
            return Verdict::Refused(refuse("build missing frame witness"));
        };
        let Some(tag_v) = inputs.get(&1) else {
            return Verdict::Refused(refuse("build missing tag"));
        };
        let Some(tag) = int_tag(tag_v) else {
            return Verdict::Refused(refuse("build tag is not ℤ"));
        };
        if tag < 0 {
            return Verdict::Refused(refuse("build tag is negative"));
        }
        let frame = frame_for_build(witness);
        let ctors = frame.constructors();
        let idx = tag as usize;
        let Some(op) = ctors.get(idx) else {
            return Verdict::Refused(refuse(&format!(
                "build tag {tag} is outside constructors of {}",
                witness.frame()
            )));
        };
        let mut args = Vec::new();
        if let Some(p0) = inputs.get(&2) {
            args.push(p0.clone());
        }
        if let Some(p1) = inputs.get(&3) {
            args.push(p1.clone());
        }
        if idx == 0 {
            args.clear();
        }
        frame.apply_op(op, &args).map(|v| BTreeMap::from([(0, v)]))
    }
}
impl Reference for BuildPrim {
    fn name(&self) -> &'static str {
        "build"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("case")
    }
    fn register(&self) -> Register {
        Register::Matter
    }
}
