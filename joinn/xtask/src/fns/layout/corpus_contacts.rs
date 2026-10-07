//! Every corpus `.contact` that does not grow, parsed, with the corpus cells,
//! in path order. A growing contact is a system's body; `grow` runs it.

use joinn_dna::{Cell, Contact, parse_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::contact::corpus_files;
use crate::fns::forces::corpus_cells;
use crate::fns::workspace_root;

/// A corpus-relative path with the contact it parses to, or the refusal.
pub(crate) type ParsedContact = (String, Verdict<Contact>);

/// The cells are `corpus_cells`, the one loader contact tooling shares (A2).
pub(crate) fn corpus_contacts() -> Result<(Vec<ParsedContact>, BTreeMap<Hash, Cell>), String> {
    let corpus = workspace_root()?.join("corpus");
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames)?;
    let mut out = Vec::new();
    for path in corpus_files(&corpus, "contact")? {
        let rel = path
            .strip_prefix(&corpus)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let src = fs::read_to_string(&path).map_err(|e| format!("{rel}: {e}"))?;
        let parsed = parse_contact(&src, &frames);
        if matches!(&parsed, Verdict::Ok(c) if c.coding.grows.is_some()) {
            continue;
        }
        out.push((rel, parsed));
    }
    Ok((out, cells))
}
