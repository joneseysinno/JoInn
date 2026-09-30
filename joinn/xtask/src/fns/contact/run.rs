//! `cargo xtask contact`: two forms, one truth, on every `.contact`.

use joinn_dna::{hash, parse_contact, print_body};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::lower;
use std::fs;

use super::corpus_files::corpus_files;
use super::desc_lines::desc_lines;
use super::lowered_line::lowered_line;
use super::mutant_lines::mutant_lines;
use super::pairings::pairings;
use super::surface_line::surface_line;
use super::transcript_line::transcript_line;
use crate::fns::forces::corpus_cells;
use crate::fns::workspace_root;

/// Any inequality, or any mutant answering against §2.14, fails. The lowered
/// text goes to stdout for comparison only; it is never written.
pub(crate) fn contact() -> Result<(), String> {
    let corpus = workspace_root()?.join("corpus");
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames)?;
    let paths = corpus_files(&corpus, "contact")?;
    if paths.is_empty() {
        return Err("contact: no .contact file in the corpus".into());
    }
    let mut failures: Vec<String> = Vec::new();
    let mut report = |line: Result<String, String>| match line {
        Ok(text) => println!("{text}"),
        Err(text) => {
            println!("{text}");
            failures.push(text);
        }
    };
    for path in &paths {
        let rel = path
            .strip_prefix(&corpus)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("{rel}: no file stem"))?;
        let src = fs::read_to_string(path).map_err(|e| format!("{rel}: {e}"))?;
        let contact = match parse_contact(&src, &frames) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => {
                report(Err(format!("{rel}: refused: {}", r.reason)));
                continue;
            }
        };
        let hex = hash(&contact.coding).to_hex();
        let lowered = match lower(&contact, &cells, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => {
                report(Err(format!("{rel}: hash {hex}, refused: {}", r.reason)));
                continue;
            }
        };
        report(Ok(format!("{rel}: hash {hex}, admitted")));
        report(Ok(format!(
            "--- lowered ---\n{}--- end ---",
            print_body(&lowered.coding)
        )));
        report(lowered_line(&corpus, stem, &lowered));
        report(transcript_line(&corpus, &rel, stem));
        match desc_lines(&corpus, stem, &contact, &cells) {
            Ok(lines) => {
                for line in lines {
                    report(Ok(line));
                }
            }
            Err(e) => report(Err(e)),
        }
        report(surface_line(&contact, &lowered, &cells));
        report(pairings(&contact, &cells, &frames));
        for line in mutant_lines(&contact, &cells, &frames) {
            report(line);
        }
    }
    if !failures.is_empty() {
        return Err(format!("contact: {} failure(s)", failures.len()));
    }
    println!("contact: {} body(ies); two forms, one truth", paths.len());
    Ok(())
}
