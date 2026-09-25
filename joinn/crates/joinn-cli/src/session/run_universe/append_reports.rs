//! Append fire, body-refusal, and link-refusal lines to the transcript.

use crate::present::fill_present;
use joinn_dna::Body;
use joinn_frame::Verdict;
use joinn_host::{describe, describe_refusal};
use joinn_link::{UniverseReport, UniverseState, format_link_refusal};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn append_reports(
    out: &mut String,
    reports: &[UniverseReport],
    state: &UniverseState,
    faces: &BTreeMap<String, Body>,
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for report in reports {
        match report {
            UniverseReport::Fired {
                body: fired,
                instance,
            } => {
                if !seen.insert((fired.clone(), instance.clone())) {
                    continue;
                }
                let Some(live) = state.body(fired) else {
                    continue;
                };
                let d = match describe(live, instance) {
                    Verdict::Ok(d) => d,
                    Verdict::Refused(r) => return Err(r.reason),
                };
                let Some(face) = faces.get(fired) else {
                    continue;
                };
                if let Some(line) = fill_present(face, &d) {
                    out.push_str(&line);
                    out.push('\n');
                }
            }
            UniverseReport::Refused {
                body: refused,
                instance,
            } => {
                let Some(live) = state.body(refused) else {
                    continue;
                };
                let Some(reason) = live.last_refusal() else {
                    continue;
                };
                let d = describe_refusal(live, instance, reason);
                out.push_str("   ");
                out.push_str(&d.label);
                out.push('\n');
            }
            UniverseReport::Link(refusal) => {
                out.push_str(&format_link_refusal(refusal));
                out.push('\n');
            }
        }
    }
    Ok(())
}
