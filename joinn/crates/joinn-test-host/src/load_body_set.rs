//! One coding hash, one regulatory face in a body set.

use joinn_dna::Body;
use joinn_dna::hash;
use joinn_frame::Hash;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Insert bodies keyed by coding hash. Refuses a second body with the same
/// coding hash and a different regulatory region, naming both source paths.
pub fn load_body_set(
    bodies: Vec<(PathBuf, Body)>,
) -> Result<BTreeMap<Hash, Body>, String> {
    let mut by_hash: BTreeMap<Hash, (Body, PathBuf)> = BTreeMap::new();
    let mut ordered = bodies;
    ordered.sort_by(|a, b| {
        a.0.file_name()
            .cmp(&b.0.file_name())
            .then_with(|| a.0.as_os_str().cmp(b.0.as_os_str()))
    });
    for (path, body) in ordered {
        let id = hash(&body.coding);
        if let Some((prev, prev_path)) = by_hash.get(&id) {
            if prev.regulatory != body.regulatory {
                return Err(format!(
                    "coding hash has distinct faces: {} and {}",
                    prev_path.display(),
                    path.display()
                ));
            }
            continue;
        }
        by_hash.insert(id, (body, path));
    }
    Ok(by_hash.into_iter().map(|(h, (b, _))| (h, b)).collect())
}
