//! Record a body refusal and maybe the matching link refusal.

use joinn_frame::{Refusal, Subject};
use std::collections::BTreeMap;

use crate::Address;
use crate::universe_state::{LinkRefusal, LinkRefusalKind, UniverseReport, UniverseState};

pub(in crate::universe_state::run) fn push_body_refusal(
    alias: &str,
    r: &Refusal,
    state: &UniverseState,
    reports: &mut Vec<UniverseReport>,
    link_refusal_on_body_refuse: bool,
    delivered_this_pass: &mut BTreeMap<(String, String, u32), (String, Address)>,
) {
    let instance = match &r.subject {
        Subject::Other(inst) => inst.clone(),
        _ => match state.bodies.get(alias).and_then(|b| b.refusal_site()) {
            Some((inst, _)) => inst.to_owned(),
            None => alias.to_owned(),
        },
    };
    reports.push(UniverseReport::Refused {
        body: alias.to_owned(),
        instance: instance.clone(),
    });
    if !link_refusal_on_body_refuse {
        return;
    }
    let Some((inst, port)) = state.bodies.get(alias).and_then(|b| b.refusal_site()) else {
        return;
    };
    if let Some((link, member)) =
        delivered_this_pass.remove(&(alias.to_owned(), inst.to_owned(), port))
    {
        reports.push(UniverseReport::Link(LinkRefusal {
            link,
            body: alias.to_owned(),
            member,
            kind: LinkRefusalKind::Refused,
        }));
    }
}
