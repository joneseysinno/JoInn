//! Run bodies in alias order, then deliver in link-id order, until a quiet pass.

use joinn_frame::{Value, Verdict};
use std::collections::BTreeMap;

use crate::Address;
use crate::universe_state::{UniverseReport, UniverseState};

use crate::universe_state::run::{deliver_links::deliver_links, run_bodies::run_bodies};

impl UniverseState {
    pub fn run(&mut self) -> Verdict<Vec<UniverseReport>> {
        let mut reports = Vec::new();
        let mut delivered_this_pass: BTreeMap<(String, String, u32), (String, Address)> =
            BTreeMap::new();
        loop {
            delivered_this_pass.clear();
            if self.spent >= self.budget {
                return crate::refuse(
                    "shared step budget exhausted; acceptance is a quiescent universe",
                );
            }
            let mut changed = false;
            let mut emitted: BTreeMap<(String, String, u32), Value> = BTreeMap::new();
            let aliases: Vec<String> = self.bodies.keys().cloned().collect();
            run_bodies(
                self,
                &aliases,
                false,
                &mut emitted,
                &mut changed,
                &mut reports,
                &mut delivered_this_pass,
            );
            let linked = deliver_links(
                self,
                &emitted,
                &mut changed,
                &mut reports,
                &mut delivered_this_pass,
            );
            if linked {
                run_bodies(
                    self,
                    &aliases,
                    true,
                    &mut emitted,
                    &mut changed,
                    &mut reports,
                    &mut delivered_this_pass,
                );
            }
            if !changed {
                return Verdict::Ok(reports);
            }
        }
    }
}
