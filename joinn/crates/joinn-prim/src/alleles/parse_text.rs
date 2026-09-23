//! Text → ℤ parse.

use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;

/// Text → ℤ parse.
pub struct ParseText;

impl Oracle for ParseText {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("parse missing line"));
        };
        let Term::Text(s) = v.term() else {
            return Verdict::Refused(refuse("parse expects Text"));
        };
        IntFrame::new().parse(s).map(|n| BTreeMap::from([(1, n)]))
    }
}
