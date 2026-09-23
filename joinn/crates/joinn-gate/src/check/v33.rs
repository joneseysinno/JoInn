//! V33: a reference body's genome reaches only the floor, its cell, and seals.

mod check_simple;

pub use check_simple::check_simple;

use joinn_dna::{AlleleBody, Body, Cell, GenomeTarget};
use joinn_frame::{CheckId, Hash, Refusal, Subject, Verdict};
use std::collections::{BTreeMap, BTreeSet};

fn refuse(reason: &str) -> Refusal {
    Refusal {
        check: CheckId::V33,
        subject: Subject::Allele(String::new()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

fn refuse_named(sealed: &str, reason: &str) -> Refusal {
    Refusal {
        check: CheckId::V33,
        subject: Subject::Allele(sealed.into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

/// A seal the genome walk can follow: its cell, its sealed native, its reference body.
#[derive(Clone, Debug)]
pub struct SealEdge {
    /// Cell this seal names.
    pub cell: Hash,
    /// Sealed native.
    pub sealed: String,
    /// Reference body hash.
    pub reference: Hash,
}

/// Structural walk of one reference body. V40 and V42. Mutant 14's teeth.
pub fn check(
    body: &Body,
    sealed: &str,
    floor: &BTreeSet<String>,
    matter: &BTreeSet<String>,
    self_cell: Hash,
    seals: &[SealEdge],
    cells: &BTreeMap<Hash, Cell>,
    bodies: &BTreeMap<Hash, Body>,
) -> Verdict<()> {
    match walk(
        body,
        sealed,
        floor,
        matter,
        self_cell,
        seals,
        cells,
        bodies,
        &mut Vec::new(),
        &mut BTreeSet::new(),
    ) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    check_seal_dag(body, sealed, self_cell, seals, cells)
}

fn walk(
    body: &Body,
    sealed: &str,
    floor: &BTreeSet<String>,
    matter: &BTreeSet<String>,
    self_cell: Hash,
    seals: &[SealEdge],
    cells: &BTreeMap<Hash, Cell>,
    bodies: &BTreeMap<Hash, Body>,
    path: &mut Vec<String>,
    seen_bodies: &mut BTreeSet<Hash>,
) -> Verdict<()> {
    if !body.coding.grants.is_empty() {
        return Verdict::Refused(refuse_named(sealed, "V33: reference body holds grants"));
    }
    for g in &body.coding.genome {
        match &g.target {
            GenomeTarget::Prim(name) => {
                if name == sealed {
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!("refused by check::v33: genome names {sealed}"),
                    ));
                }
                if !floor.contains(name) {
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!("V33: genome entry prim:{name} is not in the floor"),
                    ));
                }
                if !matter.contains(name) {
                    let reg = if name == "hash" || name == "resolve" {
                        "physics"
                    } else {
                        "space"
                    };
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!(
                            "refused by check::v33: genome entry prim:{name} is {reg}, not matter"
                        ),
                    ));
                }
            }
            GenomeTarget::Cell(h) => {
                if *h == self_cell {
                    continue;
                }
                let Some(cell) = cells.get(h) else {
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!("V33: genome names unknown cell {}", h.to_hex()),
                    ));
                };
                if cell_names_native(cell, sealed) {
                    let mut named = path.clone();
                    named.push(h.to_hex());
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!(
                            "refused by check::v33: genome reaches {sealed} through cell {}",
                            named.join(" → ")
                        ),
                    ));
                }
                let Some(edge) = justifying_seal(h, cell, seals) else {
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!(
                            "V33: cell {} has no seal (native {:?})",
                            h.to_hex(),
                            first_native(cell)
                        ),
                    ));
                };
                if edge.sealed == sealed {
                    let mut named = path.clone();
                    named.push(h.to_hex());
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!(
                            "refused by check::v33: genome reaches {sealed} through cell {}",
                            named.join(" → ")
                        ),
                    ));
                }
                if !seen_bodies.insert(edge.reference) {
                    continue;
                }
                let Some(next) = bodies.get(&edge.reference) else {
                    return Verdict::Refused(refuse_named(
                        sealed,
                        &format!("V33: seal {} has no reference body loaded", edge.sealed),
                    ));
                };
                path.push(h.to_hex());
                match walk(
                    next,
                    &edge.sealed,
                    floor,
                    matter,
                    edge.cell,
                    seals,
                    cells,
                    bodies,
                    path,
                    seen_bodies,
                ) {
                    Verdict::Ok(()) => {}
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
                path.pop();
            }
        }
    }
    Verdict::Ok(())
}

fn check_seal_dag(
    body: &Body,
    sealed: &str,
    self_cell: Hash,
    seals: &[SealEdge],
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<()> {
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    collect_edges(
        body,
        sealed,
        self_cell,
        seals,
        cells,
        &mut edges,
        &mut BTreeSet::new(),
    );
    super_dag(&edges)
}

fn collect_edges(
    body: &Body,
    sealed: &str,
    self_cell: Hash,
    seals: &[SealEdge],
    cells: &BTreeMap<Hash, Cell>,
    edges: &mut BTreeMap<String, Vec<String>>,
    seen: &mut BTreeSet<String>,
) {
    if !seen.insert(sealed.to_string()) {
        return;
    }
    let mut succ = Vec::new();
    for g in &body.coding.genome {
        let GenomeTarget::Cell(h) = &g.target else {
            continue;
        };
        if *h == self_cell {
            continue;
        }
        let Some(cell) = cells.get(h) else {
            continue;
        };
        if let Some(edge) = justifying_seal(h, cell, seals) {
            if edge.sealed != sealed {
                succ.push(edge.sealed.clone());
            }
        }
    }
    succ.sort();
    succ.dedup();
    edges.insert(sealed.to_string(), succ);
}

fn super_dag(edges: &BTreeMap<String, Vec<String>>) -> Verdict<()> {
    fn visit(
        n: &str,
        edges: &BTreeMap<String, Vec<String>>,
        stack: &mut Vec<String>,
        seen: &mut BTreeSet<String>,
    ) -> Verdict<()> {
        if stack.iter().any(|s| s == n) {
            let mut cyc = stack.clone();
            cyc.push(n.to_string());
            return Verdict::Refused(refuse(&format!("V42: cyclic seals {}", cyc.join(" → "))));
        }
        if !seen.insert(n.to_string()) {
            return Verdict::Ok(());
        }
        stack.push(n.to_string());
        if let Some(succ) = edges.get(n) {
            for s in succ {
                match visit(s, edges, stack, seen) {
                    Verdict::Ok(()) => {}
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            }
        }
        stack.pop();
        Verdict::Ok(())
    }
    let mut seen = BTreeSet::new();
    for k in edges.keys() {
        match visit(k, edges, &mut Vec::new(), &mut seen) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(())
}

fn justifying_seal<'a>(h: &Hash, cell: &Cell, seals: &'a [SealEdge]) -> Option<&'a SealEdge> {
    if let Some(s) = seals.iter().find(|s| s.cell == *h) {
        return Some(s);
    }
    let native = first_native(cell)?;
    seals.iter().find(|s| s.sealed == native)
}

fn first_native(cell: &Cell) -> Option<String> {
    match cell.alleles.first().map(|a| &a.body) {
        Some(AlleleBody::Native(id)) => Some(id.0.clone()),
        _ => None,
    }
}

fn cell_names_native(cell: &Cell, sealed: &str) -> bool {
    first_native(cell).as_deref() == Some(sealed)
}
