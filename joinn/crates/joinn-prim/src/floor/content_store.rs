//! Content-address store bridging hash and resolve.

use joinn_frame::Value;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

pub(in crate::floor) fn content_store() -> &'static Mutex<BTreeMap<String, Value>> {
    static STORE: OnceLock<Mutex<BTreeMap<String, Value>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(BTreeMap::new()))
}
