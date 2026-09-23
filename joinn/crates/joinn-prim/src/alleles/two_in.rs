//! Require in-ports 0 and 1.

use joinn_frame::{Value, Verdict};
use std::collections::BTreeMap;

use super::refuse::refuse;

pub(crate) fn two_in(inputs: &BTreeMap<u32, Value>) -> Verdict<(Value, Value)> {
    match (inputs.get(&0), inputs.get(&1)) {
        (Some(a), Some(b)) => Verdict::Ok((a.clone(), b.clone())),
        _ => Verdict::Refused(refuse("missing in-ports")),
    }
}
