//! Gate 7.4 item 3 control: the subject is refused, its lasso breaks a rule,
//! or growth moves its hash.

use joinn_dna::hash;
use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Verdict};
use joinn_link::grow;
use joinn_visual::{check_lasso, layout_system};
use std::collections::BTreeMap;

use super::forces::corpus_cells;
use super::grow::seven_inputs;
use super::refused_at_admission::refused_at_admission;
use super::subject::Subject;

/// True when the system is refused at admission, or at some size 0 … 13
/// (grown by its body's seven inputs, repeated) it does not grow or lay out,
/// a lasso rule fails, or its hash differs from size 0's.
pub(crate) fn g74_lasso_control(subject: &Subject) -> bool {
    let Subject::System(s) = subject else {
        return false;
    };
    if refused_at_admission(subject) {
        return true;
    }
    let Ok(cells) = corpus_cells(&FrameRegistry::phase1()) else {
        return true;
    };
    let contacts: BTreeMap<_, _> = s
        .contacts
        .values()
        .map(|c| (hash(&c.coding), c.clone()))
        .collect();
    let Some((alias, accepts)) = s.system.coding.bodies.first().and_then(|b| {
        let grows = s.contacts.get(&b.alias)?.coding.grows.as_ref()?;
        Some((b.alias.clone(), grows.accepts))
    }) else {
        return true;
    };
    let waiting = s
        .system
        .regulatory
        .waiting
        .get(&alias)
        .copied()
        .unwrap_or(0);
    let mut inputs = Vec::new();
    for v in seven_inputs(accepts).iter().cycle().take(13) {
        let Verdict::Ok(value) = IntFrame::new().canonicalize(Term::int(*v)) else {
            return true;
        };
        inputs.push(value);
    }
    let golden = hash(&s.system.coding);
    (0..=inputs.len()).any(|n| {
        let Verdict::Ok(grown) = grow(&s.system, &contacts, &cells, inputs.get(..n).unwrap_or(&[]))
        else {
            return true;
        };
        if hash(&grown.system().coding) != golden {
            return true;
        }
        match layout_system(&s.system, &contacts, &cells, &grown, waiting) {
            Verdict::Ok(l) => matches!(check_lasso(&l), Verdict::Refused(_)),
            Verdict::Refused(_) => true,
        }
    })
}
