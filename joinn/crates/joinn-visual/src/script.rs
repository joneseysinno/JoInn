//! Test fixture: one scripted event (§2.12) through the host path.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell};
use joinn_frame::{Frame, Hash, Term, TextFrame, Verdict};
use joinn_host::{Intent, check_intent, describe_refusal};
use joinn_link::Address;
use joinn_live::BodyState;

use crate::scene::Scene;
use crate::tables::Delta;

/// `check_intent`, canonicalize, inject at `epoch`, `run`, then `apply_run` or
/// the refusal presentation. Returns the delta and the instances the run touched,
/// counted here from the reports, apart from `apply_run`.
pub(crate) fn event(
    scene: &mut Scene,
    state: &mut BodyState,
    (body, cells): (&Body, &BTreeMap<Hash, Cell>),
    (instance, text): (&str, &str),
    epoch: u64,
) -> (Delta, Vec<String>) {
    let intent = Intent {
        address: Address {
            instance: instance.to_owned(),
            port: 0,
        },
        term: Term::Text(text.to_owned()),
    };
    if let Verdict::Refused(r) = check_intent(body, cells, &intent) {
        panic!("{}", r.reason);
    }
    let value = match TextFrame::new().canonicalize(intent.term) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    if let Verdict::Refused(r) = state.inject(instance, 0, value, epoch) {
        panic!("{}", r.reason);
    }
    match state.run() {
        Verdict::Ok(reports) => {
            let mut touched: Vec<String> = Vec::new();
            for r in &reports {
                let dests = r
                    .delivered
                    .iter()
                    .filter_map(|(w, _)| w.rsplit_once('@').map(|(d, _)| d.to_owned()));
                for name in r.fired.iter().cloned().chain(dests) {
                    if !touched.contains(&name) {
                        touched.push(name);
                    }
                }
            }
            match scene.apply_run(&reports, state) {
                Verdict::Ok(d) => (d, touched),
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        Verdict::Refused(refusal) => {
            let d = describe_refusal(state, instance, &refusal.reason);
            match scene.present(&d) {
                Verdict::Ok(d) => (d, vec![instance.to_owned()]),
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
    }
}
