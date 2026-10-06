//! Grade one non-legacy item: false on real, true on mutant, false on neutral.

use std::fs;

use joinn_frame::Verdict;
use joinn_gate::GateItem;

use super::mutate::{Mutation, mutate, neutral};
use super::parse_subject::parse_subject;
use super::refused_at_admission::refused_at_admission;
use super::resolve_named;
use super::subject::Subject;

/// Admit the item's control under real / mutant / neutral, or refuse naming it.
/// Returns which half of the control answered on the mutant: `admission` when
/// the mutant's own kind's admission refuses it, else `check`.
pub(crate) fn grade_opposed(
    n: usize,
    item: &GateItem<Subject, Mutation>,
) -> Result<&'static str, String> {
    let path = resolve_named(n, item.name, item.control_artifact)?;
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let subject = parse_subject(item.control_artifact, &text)
        .map_err(|reason| format!("gate item {n} ({}): {reason}", item.name))?;
    if (item.control)(&subject) {
        return Err(format!(
            "gate item {n} ({}): control answered true on real subject",
            item.name
        ));
    }
    let mutant = match mutate(&subject, &item.opposes) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => {
            return Err(format!("gate item {n} ({}): {}", item.name, r.reason));
        }
    };
    if !(item.control)(&mutant) {
        return Err(format!(
            "gate item {n} ({}): control answered false on mutant",
            item.name
        ));
    }
    if let Some(neu) = neutral(&subject) {
        if (item.control)(&neu) {
            return Err(format!(
                "gate item {n} ({}): control answered true on neutral edit",
                item.name
            ));
        }
    }
    Ok(if refused_at_admission(&mutant) {
        "admission"
    } else {
        "check"
    })
}
