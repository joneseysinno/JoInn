//! ℤ → Text format.

use joinn_frame::{Frame, IntFrame, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::canon_text::canon_text;
use super::refuse::refuse;

/// ℤ → Text format.
pub struct FormatInt;

impl Oracle for FormatInt {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("format missing n"));
        };
        let printed = IntFrame::new().print(v);
        canon_text(printed).map(|t| BTreeMap::from([(1, t)]))
    }
}
