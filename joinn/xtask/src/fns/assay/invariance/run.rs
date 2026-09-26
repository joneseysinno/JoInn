//! One line per subject for edits 1–4, then the phenotype-reader line.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{
    BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, assay, bind, print_assay,
};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::mutate::{Mutation, mutate};
use crate::fns::subject::Subject;
use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

use super::edit_one::edit_one;
use super::phenotype::phenotype_report;

/// One line per subject for edits 1–4, then the phenotype-reader line.
pub(crate) fn assay_invariance() -> Result<String, String> {
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

    let wrap = |body: &joinn_dna::Body| -> Universe {
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
    };
    let printed = |universe: &Universe, store: &BodyStore| -> Result<Option<String>, String> {
        let bound = match bind(universe, store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(_) => return Ok(None),
        };
        match assay(universe, &bound) {
            Verdict::Ok(report) => Ok(Some(print_assay(&report))),
            Verdict::Refused(_) => Ok(None),
        }
    };
    let differ = |rel: &str, edit: &str, before: &str, after: &str| -> String {
        format!(
            "{rel}: edit {edit} moved the report\n--- before ---\n{before}--- after ---\n{after}"
        )
    };

    let mut lines = String::new();
    let mut calc_subject: Option<Subject> = None;
    let mut loop_subject: Option<Subject> = None;
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
        let subject = if path.extension().is_some_and(|e| e == "body") {
            let body = match parse_body(&text, &frames) {
                Verdict::Ok(body) => body,
                Verdict::Refused(_) => continue,
            };
            Subject::Body(body)
        } else {
            match joinn_link::parse_universe(&text) {
                Verdict::Ok(u) => Subject::Universe(u),
                Verdict::Refused(_) => continue,
            }
        };
        let universe = match &subject {
            Subject::Body(body) => wrap(body),
            Subject::Universe(u) => u.clone(),
            _ => continue,
        };
        let Some(baseline) = printed(&universe, &store)? else {
            continue;
        };
        if rel == "phase2/calculator.body" {
            calc_subject = Some(subject.clone());
        }
        if rel == "phase4/loop.universe" {
            loop_subject = Some(subject.clone());
        }

        let edit = edit_one(&subject, &store)?;
        let assay_store = match &edit.fresh {
            Some(fresh) => fresh,
            None => &store,
        };
        let Some(after) = printed(&edit.universe, assay_store)? else {
            return Err(format!("{rel}: edit 1 refused a subject that was measured"));
        };
        if after != baseline {
            return Err(differ(&rel, "1", &baseline, &after));
        }
        let edit1 = if edit.added_name {
            "1 ok (added name)"
        } else {
            "1 ok"
        };

        let mut stripped = BodyStore::new();
        for binding in &universe.coding.bodies {
            let bound = match bind(&universe, &store) {
                Verdict::Ok(bound) => bound,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let Some((body, cell_map)) = bound.get(&binding.alias) else {
                continue;
            };
            let mut cell_map = cell_map.clone();
            for cell in cell_map.values_mut() {
                cell.alleles.clear();
            }
            match stripped.insert(body.clone(), cell_map, &rel) {
                Verdict::Ok(_) => {}
                Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
                Verdict::Refused(r) => return Err(r.reason),
            }
        }
        let Some(after) = printed(&universe, &stripped)? else {
            return Err(format!("{rel}: edit 2 refused a subject that was measured"));
        };
        if after != baseline {
            return Err(differ(&rel, "2", &baseline, &after));
        }
        let edit2 = if rel == "phase2/calculator.body" {
            "allele strip ok"
        } else {
            "2 ok"
        };

        let edit3 = if let Subject::Universe(u) = &subject {
            if u.coding.lenses.len() >= 2 {
                for lens in &u.coding.lenses {
                    let name: &'static str = Box::leak(lens.name.clone().into_boxed_str());
                    let edited = match mutate(&subject, &Mutation::DropLens(name)) {
                        Verdict::Ok(s) => s,
                        Verdict::Refused(r) => return Err(format!("{rel}: {}", r.reason)),
                    };
                    let Subject::Universe(next) = edited else {
                        return Err(format!("{rel}: edit 3 changed the kind"));
                    };
                    let Some(after) = printed(&next, &store)? else {
                        return Err(format!("{rel}: edit 3 refused a subject that was measured"));
                    };
                    if after != baseline {
                        return Err(differ(&rel, "3", &baseline, &after));
                    }
                }
                "3 ok"
            } else if u.coding.lenses.len() == 1 {
                "3 n/a (one lens)"
            } else {
                "3 n/a (no lens)"
            }
        } else {
            "3 n/a (no lens)"
        };

        let edit4 = if let Subject::Universe(_) = &subject {
            let mut renamed = false;
            let aliases: Vec<String> = universe
                .coding
                .bodies
                .iter()
                .map(|b| b.alias.clone())
                .collect();
            for alias in &aliases {
                let from: &'static str = Box::leak(alias.clone().into_boxed_str());
                let to: &'static str = Box::leak(format!("{alias}_renamed").into_boxed_str());
                let edited = match mutate(&subject, &Mutation::RenameAlias(from, to)) {
                    Verdict::Ok(s) => s,
                    Verdict::Refused(r) => return Err(format!("{rel}: {}", r.reason)),
                };
                let next = match &edited {
                    Subject::Universe(u) => u.clone(),
                    Subject::Body(_) => {
                        return Err(format!("{rel}: rename alias on a body"));
                    }
                    _ => return Err(format!("{rel}: edit 4 changed the kind")),
                };
                let Some(after) = printed(&next, &store)? else {
                    return Err(format!("{rel}: edit 4 refused a subject that was measured"));
                };
                let mapped = after.replace(to, from);
                if mapped != baseline {
                    return Err(differ(&rel, "4", &baseline, &mapped));
                }
                renamed = true;
            }
            for link in &universe.coding.links {
                let from: &'static str = Box::leak(link.id.clone().into_boxed_str());
                let to: &'static str = Box::leak(format!("{}_renamed", link.id).into_boxed_str());
                let edited = match mutate(&subject, &Mutation::RenameLink(from, to)) {
                    Verdict::Ok(s) => s,
                    Verdict::Refused(r) => return Err(format!("{rel}: {}", r.reason)),
                };
                let Subject::Universe(next) = edited else {
                    return Err(format!("{rel}: edit 4 changed the kind"));
                };
                let Some(after) = printed(&next, &store)? else {
                    return Err(format!("{rel}: edit 4 refused a subject that was measured"));
                };
                let mapped = after.replace(to, from);
                if mapped != baseline {
                    return Err(differ(&rel, "4", &baseline, &mapped));
                }
                renamed = true;
            }
            if renamed { "4 ok" } else { "4 n/a (body)" }
        } else {
            "4 n/a (body)"
        };

        lines.push_str(&format!("{rel}: {edit1}, {edit2}, {edit3}, {edit4}\n"));
    }

    let mut accepted = false;
    for (rel, subject) in [
        ("phase2/calculator.body", calc_subject),
        ("phase4/loop.universe", loop_subject),
    ] {
        let Some(subject) = subject else {
            return Err(format!(
                "phenotype reader had no {rel}; acceptance is that subject binding"
            ));
        };
        let baseline = match &subject {
            Subject::Body(body) => wrap(body),
            Subject::Universe(u) => u.clone(),
            _ => {
                return Err(format!(
                    "phenotype reader: {rel} is not a body or a universe"
                ));
            }
        };
        let before_bound = match bind(&baseline, &store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let before = phenotype_report(&baseline, &before_bound)?;
        let edit = edit_one(&subject, &store)?;
        let assay_store = match &edit.fresh {
            Some(fresh) => fresh,
            None => &store,
        };
        let after_bound = match bind(&edit.universe, assay_store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let after = phenotype_report(&edit.universe, &after_bound)?;
        let verdict = if before == after {
            accepted = true;
            format!("phenotype reader: accepted at edit 1 on {rel}")
        } else {
            format!("phenotype reader: refused at edit 1 on {rel} (ok)")
        };
        lines.push_str(&verdict);
        lines.push('\n');
    }
    if accepted {
        return Err(lines);
    }
    Ok(lines)
}
