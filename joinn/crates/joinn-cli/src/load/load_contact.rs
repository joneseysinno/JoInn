//! Load a `.contact` by path and derive the body it runs as.

use joinn_dna::{Body, Cell, parse_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::lower;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// `target` is a path: from the working directory, else from the directory
/// that holds `corpus/`. The contact is admitted and lowered against `cells`.
pub(crate) fn load_contact(
    corpus: &Path,
    target: &str,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<Body, String> {
    let given = Path::new(target);
    let path = if given.is_file() {
        given.to_path_buf()
    } else {
        corpus
            .parent()
            .map(|root| root.join(target))
            .filter(|p| p.is_file())
            .ok_or_else(|| format!("no contact file at {target}"))?
    };
    let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let frames = FrameRegistry::phase1();
    let contact = match parse_contact(&src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match lower(&contact, cells, &frames) {
        Verdict::Ok(body) => Ok(body),
        Verdict::Refused(r) => Err(r.reason),
    }
}
