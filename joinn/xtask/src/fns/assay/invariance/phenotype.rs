//! A fake assay that prints every name and label it was handed.

use joinn_frame::Verdict;
use joinn_link::{Bound, Universe, assay, print_assay};

/// The real report, then each name and label from `bound` and from the universe.
pub(crate) fn phenotype_report(universe: &Universe, bound: &Bound) -> Result<String, String> {
    let report = match assay(universe, bound) {
        Verdict::Ok(report) => report,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut out = print_assay(&report);
    for (alias, (body, _)) in bound.iter() {
        for (key, value) in &body.regulatory.names {
            out.push_str(&format!("{alias} name {key} {value}\n"));
        }
        for (key, value) in &body.regulatory.labels {
            out.push_str(&format!("{alias} label {key} {value}\n"));
        }
    }
    for (key, value) in &universe.regulatory.names {
        out.push_str(&format!("universe name {key} {value}\n"));
    }
    for (key, value) in &universe.regulatory.labels {
        out.push_str(&format!("universe label {key} {value}\n"));
    }
    Ok(out)
}
