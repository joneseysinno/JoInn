//! Load `corpus/phase74/<name>.system` with its bodies and its golden hash.

use joinn_dna::parse_system;
use joinn_frame::Verdict;
use std::fs;

use super::CorpusSystem;
use crate::fns::layout::corpus_growing_contacts;
use crate::fns::{hashes_path, workspace_root};

/// Its one body must be a corpus growing contact; its hash row must exist.
pub(crate) fn corpus_system(name: &str) -> Result<CorpusSystem, String> {
    let file = format!("{name}.system");
    let path = workspace_root()?.join("corpus").join("phase74").join(&file);
    let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let system = match parse_system(&src) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("{file}: {}", r.reason)),
    };
    let contacts = corpus_growing_contacts()?;
    let accepts = system
        .coding
        .bodies
        .first()
        .and_then(|b| contacts.get(&b.contact))
        .and_then(|c| c.coding.grows.as_ref())
        .map(|g| g.accepts)
        .ok_or_else(|| format!("{file}: its body is no corpus growing contact"))?;
    let hashes = fs::read_to_string(hashes_path()?).map_err(|e| e.to_string())?;
    let golden = hashes
        .lines()
        .find_map(|l| l.strip_prefix(&format!("{file} ")))
        .map(|h| h.trim().to_owned())
        .ok_or_else(|| format!("{file}: no row in hashes.txt"))?;
    Ok(CorpusSystem {
        name: name.to_owned(),
        system,
        contacts,
        accepts,
        golden,
    })
}
