//! The test host's descriptions of a contact body under the script, against
//! the stem's description goldens.

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_host::{Role, print_description};
use joinn_test_host::run_contact;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::script_events::script_events;

/// `<stem>.desc` is the last cell fire (the sum, after the last event);
/// `<stem>_refusal.desc` is the first refusal (after the first event).
pub(super) fn desc_lines(
    corpus: &Path,
    stem: &str,
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<Vec<String>, String> {
    let cap = match run_contact(
        contact,
        cells.clone(),
        joinn_prim::sealed_natives(),
        script_events(),
    ) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(format!("descriptions: refused: {}", r.reason)),
    };
    let fired = cap.descriptions.iter().rev().find(|d| d.role == Role::Cell);
    let refused = cap.descriptions.iter().find(|d| d.role == Role::Refusal);
    let mut lines = Vec::new();
    for (name, got) in [
        (format!("{stem}.desc"), fired),
        (format!("{stem}_refusal.desc"), refused),
    ] {
        let path = corpus.join("descriptions").join(&name);
        let want = fs::read_to_string(&path)
            .map_err(|e| format!("descriptions: {}: {e}", path.display()))?
            .replace("\r\n", "\n");
        let Some(got) = got else {
            return Err(format!("{name}: the script described nothing to compare"));
        };
        let got = print_description(got);
        if got != want {
            return Err(format!("{name} differs:\n{got}"));
        }
        lines.push(format!("{name} equal"));
    }
    Ok(lines)
}
