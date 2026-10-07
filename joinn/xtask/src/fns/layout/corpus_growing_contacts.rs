//! Every corpus `.contact` that grows, by coding hash.

use joinn_dna::{Contact, hash, parse_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::contact::corpus_files;
use crate::fns::workspace_root;

/// The bodies corpus systems bind. A refused file is left to `corpus verify`.
pub(crate) fn corpus_growing_contacts() -> Result<BTreeMap<Hash, Contact>, String> {
    let corpus = workspace_root()?.join("corpus");
    let frames = FrameRegistry::phase1();
    let mut out = BTreeMap::new();
    for path in corpus_files(&corpus, "contact")? {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if let Verdict::Ok(c) = parse_contact(&src, &frames) {
            if c.coding.grows.is_some() {
                out.insert(hash(&c.coding), c);
            }
        }
    }
    Ok(out)
}
