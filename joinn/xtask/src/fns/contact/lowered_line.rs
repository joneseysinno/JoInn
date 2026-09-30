//! The lowered body against the corpus `.body` of the same stem.

use joinn_dna::{Body, hash, parse_body, print_body};
use joinn_frame::{FrameRegistry, Verdict};
use std::fs;
use std::path::Path;

use super::corpus_files::corpus_files;

/// `lowered = <rel> (<hash>…)` when the two canonical texts are byte-identical;
/// otherwise the first line where they differ.
pub(crate) fn lowered_line(corpus: &Path, stem: &str, lowered: &Body) -> Result<String, String> {
    let want_name = format!("{stem}.body");
    let hits: Vec<_> = corpus_files(corpus, "body")?
        .into_iter()
        .filter(|p| p.file_name().is_some_and(|n| n == want_name.as_str()))
        .collect();
    let [path] = hits.as_slice() else {
        return Err(format!(
            "lowered: {} corpus files are named {want_name}; acceptance is exactly one",
            hits.len()
        ));
    };
    let rel = path
        .strip_prefix(corpus)
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .replace('\\', "/");
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let wired = match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(format!("lowered: {rel} is refused: {}", r.reason)),
    };
    let got = print_body(&lowered.coding);
    let want = print_body(&wired.coding);
    if got == want {
        let hex = hash(&wired.coding).to_hex();
        return Ok(format!("lowered = {rel} ({}…)", &hex[..8]));
    }
    let mut got_lines = got.lines();
    let mut want_lines = want.lines();
    for n in 1.. {
        match (got_lines.next(), want_lines.next()) {
            (Some(g), Some(w)) if g == w => {}
            (None, None) => break,
            (g, w) => {
                return Err(format!(
                    "lowered differs from {rel} at line {n}: lowered `{}`, {rel} `{}`",
                    g.unwrap_or("(end)"),
                    w.unwrap_or("(end)")
                ));
            }
        }
    }
    Err(format!("lowered differs from {rel}"))
}
