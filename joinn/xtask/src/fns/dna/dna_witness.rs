//! `cargo xtask dna-witness`: write Phase 8's witnesses once.

use joinn_frame::keyed_hash;
use joinn_strand::TAG_DNA;

use super::super::{corpus_artifact, workspace_root};
use super::strand_witnesses;

/// Each witness absent from the corpus is written; one present with the same
/// bytes is left alone; one present with other bytes is refused, never
/// rewritten. Prints each witness as its `hashes.txt` line.
pub(crate) fn dna_witness() -> Result<(), String> {
    let root = workspace_root()?;
    for (name, text) in strand_witnesses()? {
        let path = corpus_artifact(&root, &name);
        let state = match std::fs::read_to_string(&path) {
            Ok(on_disk) if on_disk.replace("\r\n", "\n") == text => "present",
            Ok(_) => {
                return Err(format!(
                    "dna-witness: {name} exists with other bytes; acceptance is a witness written once, never rewritten"
                ));
            }
            Err(_) => {
                if let Some(dir) = path.parent() {
                    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
                }
                std::fs::write(&path, &text).map_err(|e| format!("{}: {e}", path.display()))?;
                "written"
            }
        };
        println!(
            "{name} {}   # {state}",
            keyed_hash(TAG_DNA, text.as_bytes()).to_hex()
        );
    }
    Ok(())
}
