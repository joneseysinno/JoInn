//! Does the assay catch anything the other checks do not?

use joinn_dna::{AlleleBody, GenomeTarget, hash, parse_body, parse_cell, print_formula};
use joinn_frame::{FrameRegistry, Term, Verdict};
use joinn_gate::{Budget, Gate};
use joinn_host::{Address, print_description};
use joinn_link::{
    BodyStore, Universe, assay, assemble_universe, bind, check_law4, check_lenses, check_link_types,
};
use joinn_test_host::{RawEvent, run_universe};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::mutate::{Mutation, mutate};
use crate::fns::subject::Subject;
use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

/// One line per check on `loop.universe` and its `fmt_twin` mutant, then who distinguished them.
pub(crate) fn decoration() -> Result<String, String> {
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
    let mut body_paths = Vec::new();
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
            } else if name.extension().is_some_and(|e| e == "body") {
                body_paths.push(name);
            }
        }
    }
    body_paths.sort();
    let mut store = BodyStore::new();
    for path in &body_paths {
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

    let loop_path = corpus.join("phase4").join("loop.universe");
    let loop_text = fs::read_to_string(&loop_path).map_err(|e| e.to_string())?;
    let subject = match joinn_link::parse_universe(&loop_text) {
        Verdict::Ok(u) => Subject::Universe(u),
        Verdict::Refused(r) => return Err(r.reason),
    };
    let twin_path = corpus.join("phase4").join("fmt_twin.body");
    let twin_text = fs::read_to_string(&twin_path).map_err(|e| e.to_string())?;
    let twin_body = match parse_body(&twin_text, &frames) {
        Verdict::Ok(body) => body,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let twin_hex = hash(&twin_body.coding).to_hex();
    let twin_static: &'static str = Box::leak(twin_hex.into_boxed_str());
    let mutant = match mutate(&subject, &Mutation::SwapBinding("fmt", twin_static)) {
        Verdict::Ok(Subject::Universe(u)) => u,
        Verdict::Ok(_) => {
            return Err("SwapBinding changed the kind; acceptance is a universe".into());
        }
        Verdict::Refused(r) => return Err(r.reason),
    };
    let Subject::Universe(subject) = subject else {
        return Err("loop.universe did not parse as a universe".into());
    };

    let verdict = |result: Verdict<()>| -> String {
        match result {
            Verdict::Ok(()) => "admitted".to_string(),
            Verdict::Refused(r) => format!("refused: {}", r.reason),
        }
    };
    let evaluate = |universe: &Universe| -> Result<Vec<(&'static str, String)>, String> {
        let bound = bind(universe, &store);
        let mut rows = Vec::new();

        let gate_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => {
                let mut gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
                let mut refusals = Vec::new();
                for (alias, (body, cell_map)) in bound.iter() {
                    for entry in &body.coding.genome {
                        let GenomeTarget::Cell(id) = &entry.target else {
                            continue;
                        };
                        let Some(cell) = cell_map.get(id) else {
                            refusals.push(format!("{alias} {} was not supplied", id.short_hex()));
                            continue;
                        };
                        if let Some(allele) = cell.alleles.first() {
                            if let AlleleBody::Native(id_native) = &allele.body {
                                if let Some(oracle) = gate.native_arc(&id_native.0) {
                                    gate.register_cell_oracle(*id, oracle);
                                }
                            }
                        }
                        match gate.admit_cell(cell, None) {
                            Verdict::Ok(_) => {}
                            Verdict::Refused(r) => {
                                refusals.push(format!("{alias} {}: {}", id.short_hex(), r.reason));
                            }
                        }
                    }
                }
                if refusals.is_empty() {
                    "admitted".to_string()
                } else {
                    format!("refused: {}", refusals.join("; "))
                }
            }
        };
        rows.push(("gate admission", gate_answer));

        let insert_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => {
                let mut fresh = BodyStore::new();
                let mut refusals = Vec::new();
                for (alias, (body, cell_map)) in bound.iter() {
                    match fresh.insert(body.clone(), cell_map.clone(), alias) {
                        Verdict::Ok(_) => {}
                        Verdict::Refused(r) => {
                            refusals.push(format!("{alias}: {}", r.reason));
                        }
                    }
                }
                if refusals.is_empty() {
                    "admitted".to_string()
                } else {
                    format!("refused: {}", refusals.join("; "))
                }
            }
        };
        rows.push(("insert", insert_answer));

        let bind_answer = match &bound {
            Verdict::Ok(_) => "admitted".to_string(),
            Verdict::Refused(r) => format!("refused: {}", r.reason),
        };
        rows.push(("bind", bind_answer));

        let assemble_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => verdict(assemble_universe(universe, bound)),
        };
        rows.push(("assemble", assemble_answer));

        let types_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => verdict(check_link_types(universe, bound)),
        };
        rows.push(("check_link_types", types_answer));

        rows.push(("check_law4", verdict(check_law4(universe))));
        rows.push(("check_lenses", verdict(check_lenses(universe))));

        let contract_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => {
                let mut declared = Vec::new();
                for (alias, (body, cell_map)) in bound.iter() {
                    for entry in &body.coding.genome {
                        let GenomeTarget::Cell(id) = &entry.target else {
                            continue;
                        };
                        let Some(cell) = cell_map.get(id) else {
                            continue;
                        };
                        for (port, formula) in &cell.coding.contract.require {
                            declared
                                .push(format!("{alias} require {port} {}", print_formula(formula)));
                        }
                        for (port, formula) in &cell.coding.contract.ensure {
                            declared
                                .push(format!("{alias} ensure {port} {}", print_formula(formula)));
                        }
                    }
                }
                if declared.is_empty() {
                    "none declared".to_string()
                } else {
                    declared.join("; ")
                }
            }
        };
        rows.push(("require/ensure", contract_answer));

        let run_answer = match bind(universe, &store) {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(owned) => {
                let rounds = vec![vec![
                    (
                        "calc".to_string(),
                        RawEvent {
                            address: Address {
                                instance: "cli_a".to_string(),
                                port: 0,
                            },
                            term: Term::text("2"),
                        },
                    ),
                    (
                        "calc".to_string(),
                        RawEvent {
                            address: Address {
                                instance: "cli_b".to_string(),
                                port: 0,
                            },
                            term: Term::text("3"),
                        },
                    ),
                ]];
                match run_universe(universe, owned, joinn_prim::sealed_natives(), rounds) {
                    Verdict::Refused(r) => format!("refused: {}", r.reason),
                    Verdict::Ok(capture) => {
                        if capture.descriptions.is_empty() && capture.far_side.is_empty() {
                            "no report".to_string()
                        } else {
                            let mut parts = Vec::new();
                            for description in &capture.descriptions {
                                parts.push(print_description(description).replace('\n', " "));
                            }
                            for refusal in &capture.far_side {
                                parts.push(format!(
                                    "far {} {} {} {:?}",
                                    refusal.link,
                                    refusal.body,
                                    refusal.member.printed(),
                                    refusal.kind
                                ));
                            }
                            parts.join(" || ")
                        }
                    }
                }
            }
        };
        rows.push(("run", run_answer));

        let assay_answer = match &bound {
            Verdict::Refused(r) => format!("refused: {}", r.reason),
            Verdict::Ok(bound) => match assay(universe, bound) {
                Verdict::Ok(report) => format!("H₁: {}", report.b1),
                Verdict::Refused(r) => format!("refused: {}", r.reason),
            },
        };
        rows.push(("assay", assay_answer));
        Ok(rows)
    };

    let subject_rows = evaluate(&subject)?;
    let mutant_rows = evaluate(&mutant)?;
    if subject_rows.len() != mutant_rows.len() {
        return Err(format!(
            "decoration ran {} checks on the subject and {} on the mutant; acceptance is the same list",
            subject_rows.len(),
            mutant_rows.len()
        ));
    }
    let mut lines = String::new();
    let mut distinguishing = Vec::new();
    for ((name, left), (right_name, right)) in subject_rows.iter().zip(mutant_rows.iter()) {
        if name != right_name {
            return Err(format!(
                "decoration check {name} was paired with {right_name}; acceptance is the same order"
            ));
        }
        let both_refused = left.starts_with("refused") && right.starts_with("refused");
        let flag = if both_refused || left == right {
            "same"
        } else {
            "differs"
        };
        if flag == "differs" {
            distinguishing.push(*name);
        }
        lines.push_str(&format!("{name}: {left} | {right} | {flag}\n"));
    }
    lines.push_str(&format!("distinguishing: {}\n", distinguishing.join(", ")));
    Ok(lines)
}
