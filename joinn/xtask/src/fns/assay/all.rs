//! `cargo xtask assay --all`: every corpus body and universe.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{
    BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, assay, bind, print_assay,
};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

/// One report per subject that parses. A refusal is a line, not a stop.
pub(crate) fn assay_all() -> Result<String, String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let corpus = root.join("corpus");
    let mut cell_paths = Vec::new();
    walk_cells(&corpus, &mut cell_paths)?;
    let mut cells = BTreeMap::new();
    for cell_path in &cell_paths {
        let text = fs::read_to_string(cell_path).map_err(|e| e.to_string())?;
        let cell = match parse_cell(&text, &frames) {
            Verdict::Ok(cell) => cell,
            Verdict::Refused(_) => continue,
        };
        cells.entry(hash(&cell.coding)).or_insert(cell);
    }
    let mut dirs = vec![corpus.clone()];
    let mut subjects = Vec::new();
    while let Some(dir) = dirs.pop() {
        let read = fs::read_dir(&dir).map_err(|e| e.to_string())?;
        let mut names = Vec::new();
        for ent in read {
            names.push(ent.map_err(|e| e.to_string())?.path());
        }
        names.sort();
        for name in names {
            if name.is_dir() {
                if name.file_name().is_some_and(|n| n == "counterfeit") {
                    continue;
                }
                dirs.push(name);
            } else if name
                .extension()
                .is_some_and(|e| e == "body" || e == "universe")
            {
                subjects.push(name);
            }
        }
    }
    subjects.sort();
    let mut store = BodyStore::new();
    for path in &subjects {
        if path.extension().is_none_or(|e| e != "body") {
            continue;
        }
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let body = match parse_body(&text, &frames) {
            Verdict::Ok(body) => body,
            Verdict::Refused(_) => continue,
        };
        let source = path.display().to_string();
        match store.insert(body, cells.clone(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }

    let mut lines = String::new();
    for path in &subjects {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let rel = match path.strip_prefix(&corpus) {
            Ok(rel) => rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("/"),
            Err(_) => path.display().to_string(),
        };
        let universe = if path.extension().is_some_and(|e| e == "body") {
            let body = match parse_body(&text, &frames) {
                Verdict::Ok(body) => body,
                Verdict::Refused(r) => {
                    lines.push_str(&format!("{rel}: {}\n", r.reason));
                    continue;
                }
            };
            Universe {
                coding: UniverseCoding {
                    codex: 1,
                    bodies: vec![BodyBinding {
                        hash: hash(&body.coding),
                        alias: "body".to_string(),
                    }],
                    links: Vec::new(),
                    cross_wires: Vec::new(),
                    grants: BTreeMap::new(),
                    lenses: Vec::new(),
                },
                regulatory: UniverseRegulatory::default(),
            }
        } else {
            match joinn_link::parse_universe(&text) {
                Verdict::Ok(u) => u,
                Verdict::Refused(r) => {
                    lines.push_str(&format!("{rel}: {}\n", r.reason));
                    continue;
                }
            }
        };
        let bound = match bind(&universe, &store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => {
                lines.push_str(&format!("{rel}: {}\n", r.reason));
                continue;
            }
        };
        match assay(&universe, &bound) {
            Verdict::Ok(report) => {
                lines.push_str(&rel);
                lines.push('\n');
                lines.push_str(&print_assay(&report));
            }
            Verdict::Refused(r) => lines.push_str(&format!("{rel}: {}\n", r.reason)),
        }
    }
    Ok(lines)
}
