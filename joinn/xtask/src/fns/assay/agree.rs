//! Both derivations on every corpus subject that binds.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{
    AssayReport, BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, assay,
    assay_reference, bind, print_assay,
};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

/// One line per subject, then the summary. A real disagreement exits with both reports.
pub(crate) fn assay_agree() -> Result<String, String> {
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

    let same = |fast: &AssayReport, reference: &AssayReport| -> bool {
        let euler = |report: &AssayReport| {
            let left = i64::from(report.v) - i64::from(report.e) + i64::from(report.f);
            let right = i64::from(report.b0) - i64::from(report.b1) + i64::from(report.b2);
            left == right
        };
        fast.regions == reference.regions
            && fast.islands == reference.islands
            && fast.b1 == reference.b1
            && fast.b2 == reference.b2
            && fast.filled == reference.filled
            && fast.open == reference.open
            && euler(fast)
            && euler(reference)
    };

    let mut lines = String::new();
    let mut n = 0_u32;
    let mut m = 0_u32;
    let mut plant: Option<(AssayReport, AssayReport)> = None;
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
                Verdict::Refused(_) => continue,
            };
            let id = hash(&body.coding);
            Universe {
                coding: UniverseCoding {
                    codex: 1,
                    bodies: vec![BodyBinding {
                        hash: id,
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
                Verdict::Refused(_) => continue,
            }
        };
        let bound = match bind(&universe, &store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => {
                lines.push_str(&format!("{rel}: not measured: {}\n", r.reason));
                m = m.saturating_add(1);
                continue;
            }
        };
        let fast = match assay(&universe, &bound) {
            Verdict::Ok(report) => Ok(report),
            Verdict::Refused(r) => Err(r.reason),
        };
        let reference = match assay_reference(&universe, &bound) {
            Verdict::Ok(report) => Ok(report),
            Verdict::Refused(r) => Err(r.reason),
        };
        let (fast, reference) = match (fast, reference) {
            (Ok(fast), Ok(reference)) => (fast, reference),
            (Err(fast_reason), Err(reference_reason)) => {
                let reason = if fast_reason == reference_reason {
                    fast_reason
                } else {
                    format!("{fast_reason} | {reference_reason}")
                };
                lines.push_str(&format!("{rel}: not measured: {reason}\n"));
                m = m.saturating_add(1);
                continue;
            }
            (Ok(_), Err(_)) => {
                return Err(format!(
                    "{rel}: reference derivation refused a subject the fast derivation measured"
                ));
            }
            (Err(_), Ok(_)) => {
                return Err(format!(
                    "{rel}: fast derivation refused a subject the reference derivation measured"
                ));
            }
        };
        if !same(&fast, &reference) {
            return Err(format!(
                "{rel}: truth violation\n--- fast ---\n{}--- reference ---\n{}",
                print_assay(&fast),
                print_assay(&reference)
            ));
        }
        if rel == "phase2/calculator.body" {
            plant = Some((fast, reference));
        }
        lines.push_str(&format!("{rel}: agree\n"));
        n = n.saturating_add(1);
    }

    let Some((fast, reference)) = plant else {
        return Err(
            "injected disagreement had no calculator subject; acceptance is phase2/calculator.body binding"
                .into(),
        );
    };
    let mut dropped = fast.clone();
    if dropped.filled.pop().is_none() {
        return Err(
            "injected disagreement found no filling on calculator.body; acceptance is two roundtrip fillings"
                .into(),
        );
    }
    if same(&dropped, &reference) {
        return Err(
            "injected disagreement was accepted; acceptance is a refusal as truth violation".into(),
        );
    }
    let mut raised = fast;
    raised.b1 = raised.b1.saturating_add(1);
    if same(&raised, &reference) {
        return Err(
            "injected disagreement on b₁ was accepted; acceptance is a refusal as truth violation"
                .into(),
        );
    }
    lines.push_str(&format!(
        "assay agree: {n} subject(s) agree, {m} not measured; injected disagreements: 2 refused as truth violation (ok)\n"
    ));
    Ok(lines)
}
