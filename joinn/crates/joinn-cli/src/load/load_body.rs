//! Load a body by file stem.

use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use std::fs;
use std::path::Path;

/// Load the unique `{name}.body` under `corpus`.
pub(crate) fn load_body(corpus: &Path, name: &str) -> Result<joinn_dna::Body, String> {
    if name.is_empty() || name.contains(['/', '\\', '.']) {
        return Err(format!("body name `{name}` is not a single stem"));
    }
    let want = format!("{name}.body");
    let mut hits = Vec::new();
    let mut dirs = vec![corpus.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let mut entries: Vec<_> = rd
            .map(|e| e.map_err(|err| err.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.file_name());
        for ent in entries {
            let path = ent.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some(want.as_str()) {
                hits.push(path);
            }
        }
    }
    hits.sort_by(|a, b| a.as_os_str().cmp(b.as_os_str()));
    let path = match hits.len() {
        1 => hits.remove(0),
        0 => return Err(format!("no body file named {want}")),
        _ => {
            let listed = hits
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(format!(
                "body name `{name}` matches more than a single file: {listed}"
            ));
        }
    };
    let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => Ok(b),
        Verdict::Refused(r) => Err(r.reason),
    }
}
