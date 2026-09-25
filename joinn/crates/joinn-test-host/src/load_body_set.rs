//! One coding hash, one regulatory face in a body set.

use joinn_dna::{Body, hash};
use joinn_frame::{Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Insert bodies keyed by coding hash. Refuses a second body with the same
/// coding hash and a different regulatory region, naming both source paths.
pub fn load_body_set(bodies: Vec<(PathBuf, Body)>) -> Result<BTreeMap<Hash, Body>, String> {
    let mut store = BodyStore::new();
    let mut ordered = bodies;
    ordered.sort_by(|a, b| {
        a.0.file_name()
            .cmp(&b.0.file_name())
            .then_with(|| a.0.as_os_str().cmp(b.0.as_os_str()))
    });
    let mut out = BTreeMap::new();
    for (path, body) in ordered {
        let id = hash(&body.coding);
        let source = path.display().to_string();
        match store.insert(body.clone(), BTreeMap::new(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
        out.insert(id, body);
    }
    Ok(out)
}
