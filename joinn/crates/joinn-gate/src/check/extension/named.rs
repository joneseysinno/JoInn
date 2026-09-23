//! Bindings map keyed by port number string.

use joinn_frame::Value;
use std::collections::BTreeMap;

pub(in crate::check::extension) fn named(inputs: &BTreeMap<u32, Value>) -> BTreeMap<String, Value> {
    inputs
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}
