//! Run bodies in alias order, then deliver in link-id order, until a quiet pass.

use joinn_frame::{Value, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use crate::Address;
use crate::universe_state::{UniverseReport, UniverseState};

use crate::universe_state::run::{deliver_links::deliver_links, run_bodies::run_bodies};

impl UniverseState {
    pub fn run(&mut self) -> Verdict<Vec<UniverseReport>> {
        let mut reports = Vec::new();
        let mut delivered_this_pass: BTreeMap<(String, String, u32), (String, Address)> =
            BTreeMap::new();
        // Emissions from the sweep that follows a delivery. The next pass delivers them.
        let mut carried: BTreeMap<(String, String, u32), Value> = BTreeMap::new();
        // A body reported Refused is not run again in this call.
        let mut already_refused: BTreeSet<String> = BTreeSet::new();
        loop {
            delivered_this_pass.clear();
            if self.spent >= self.budget {
                return crate::refuse(
                    "shared step budget exhausted; acceptance is a quiescent universe",
                );
            }
            let mut changed = false;
            let mut emitted = std::mem::take(&mut carried);
            let aliases: Vec<String> = self
                .bodies
                .keys()
                .filter(|alias| !already_refused.contains(*alias))
                .cloned()
                .collect();
            run_bodies(
                self,
                &aliases,
                false,
                &mut emitted,
                &mut changed,
                &mut reports,
                &mut delivered_this_pass,
            );
            for report in &reports {
                if let UniverseReport::Refused { body, .. } = report {
                    already_refused.insert(body.clone());
                }
            }
            let linked = deliver_links(
                self,
                &emitted,
                &mut changed,
                &mut reports,
                &mut delivered_this_pass,
            );
            if linked {
                let delivered_to: BTreeSet<&str> = delivered_this_pass
                    .keys()
                    .map(|(body, _, _)| body.as_str())
                    .collect();
                let recipients: Vec<String> = aliases
                    .iter()
                    .filter(|alias| delivered_to.contains(alias.as_str()))
                    .filter(|alias| !already_refused.contains(*alias))
                    .cloned()
                    .collect();
                let mut follow = BTreeMap::new();
                run_bodies(
                    self,
                    &recipients,
                    true,
                    &mut follow,
                    &mut changed,
                    &mut reports,
                    &mut delivered_this_pass,
                );
                carried = follow;
                for report in &reports {
                    if let UniverseReport::Refused { body, .. } = report {
                        already_refused.insert(body.clone());
                    }
                }
            }
            if !changed && carried.is_empty() {
                return Verdict::Ok(reports);
            }
        }
    }
}
