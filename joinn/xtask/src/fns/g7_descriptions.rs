//! The test host's descriptions of the contact calculator under the script,
//! against the description goldens byte for byte and `hashes.txt` by hash.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_host::{Role, hash_description, print_description};
use joinn_test_host::run_contact;

use super::contact::script_events;

/// `calculator.desc` is the last cell fire (the sum, after event 3);
/// `calculator_refusal.desc` is the first refusal (after event 1). Each
/// inequality is one failure.
pub(crate) fn g7_descriptions(
    corpus: &Path,
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
) -> Vec<String> {
    let cap = match run_contact(
        contact,
        cells.clone(),
        joinn_prim::sealed_natives(),
        script_events(),
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return vec![format!("descriptions: refused: {}", r.reason)],
    };
    let hashes = match fs::read_to_string(corpus.join("hashes.txt")) {
        Ok(text) => text,
        Err(e) => return vec![format!("descriptions: hashes.txt: {e}")],
    };
    let fired = cap.descriptions.iter().rev().find(|d| d.role == Role::Cell);
    let refused = cap.descriptions.iter().find(|d| d.role == Role::Refusal);
    let mut failures = Vec::new();
    for (name, got) in [
        ("calculator.desc", fired),
        ("calculator_refusal.desc", refused),
    ] {
        let Some(got) = got else {
            failures.push(format!("{name}: the script described nothing to compare"));
            continue;
        };
        let path = corpus.join("descriptions").join(name);
        match fs::read_to_string(&path) {
            Ok(want) => {
                let printed = print_description(got);
                if printed != want.replace("\r\n", "\n") {
                    failures.push(format!("{name} differs byte for byte:\n{printed}"));
                }
            }
            Err(e) => failures.push(format!("{name}: {}: {e}", path.display())),
        }
        let got_hex = hash_description(got).to_hex();
        let want_hex = hashes
            .lines()
            .find_map(|l| l.strip_prefix(name).and_then(|rest| rest.strip_prefix(' ')))
            .map(str::trim);
        match want_hex {
            Some(w) if w == got_hex => {}
            Some(w) => failures.push(format!("{name}: hash {got_hex}, hashes.txt {w}")),
            None => failures.push(format!("{name}: no row in hashes.txt")),
        }
    }
    failures
}
