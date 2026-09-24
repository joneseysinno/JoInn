//! Alternate body steps and link deliveries until nothing changes.

use joinn_frame::{Value, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use crate::capability::check_capability;
use crate::universe::{Mark, Order};
use crate::universe_state::{LinkRefusal, LinkRefusalKind, UniverseReport, UniverseState};
use crate::Address;

impl UniverseState {
    /// Run bodies in alias order, then deliver in link-id order, until a quiet pass.
    pub fn run(&mut self) -> Verdict<Vec<UniverseReport>> {
        let mut reports = Vec::new();
        let mut pending: BTreeMap<String, (String, Address)> = BTreeMap::new();
        loop {
            if self.spent >= self.budget {
                return crate::refuse(
                    "shared step budget exhausted; acceptance is a quiescent universe",
                );
            }
            let mut changed = false;
            let mut emitted: BTreeMap<(String, String, u32), Value> = BTreeMap::new();
            let aliases: Vec<String> = self.bodies.keys().cloned().collect();
            for alias in &aliases {
                let used = self.bodies.get(alias).map(|b| b.steps()).unwrap_or(0);
                let remaining = self.budget.saturating_sub(self.spent); // allow(vocab): budget arithmetic on u64, not the subtract concept
                if let Some(body) = self.bodies.get_mut(alias) {
                    body.set_budget(used.saturating_add(remaining));
                }
                let ran = match self.bodies.get_mut(alias) {
                    Some(body) => body.run(),
                    None => continue,
                };
                let now = self.bodies.get(alias).map(|b| b.steps()).unwrap_or(used);
                self.spent = self.spent.saturating_add(now.saturating_sub(used)); // allow(vocab): budget arithmetic on u64, not the subtract concept
                match ran {
                    Verdict::Ok(steps) => {
                        for step in steps {
                            let Some(instance) = step.fired else {
                                continue;
                            };
                            changed = true;
                            reports.push(UniverseReport::Fired {
                                body: alias.clone(),
                                instance: instance.clone(),
                            });
                            if let Some(ports) = self
                                .bodies
                                .get(alias)
                                .and_then(|b| b.last_ports(&instance))
                            {
                                for (port, value) in ports {
                                    emitted.insert(
                                        (alias.clone(), instance.clone(), *port),
                                        value.clone(),
                                    );
                                }
                            }
                        }
                    }
                    Verdict::Refused(r) => {
                        if let Some((link, member)) = pending.remove(alias) {
                            reports.push(UniverseReport::Link(LinkRefusal {
                                link,
                                body: alias.clone(),
                                member,
                                kind: LinkRefusalKind::Refused,
                            }));
                            return Verdict::Ok(reports);
                        }
                        return Verdict::Refused(r);
                    }
                }
            }
            let mut links = self.universe.coding.links.clone();
            links.sort_by(|a, b| a.id.cmp(&b.id));
            let mut taken: BTreeSet<(String, String, u32)> = BTreeSet::new();
            for link in &links {
                let mut members = link.members.clone();
                if link.order != Order::Ordered {
                    members.sort();
                }
                let tails: Vec<_> = members
                    .iter()
                    .filter(|m| m.mark == Mark::Tail)
                    .cloned()
                    .collect();
                let heads: Vec<_> = members
                    .iter()
                    .filter(|m| m.mark == Mark::Head)
                    .cloned()
                    .collect();
                for tail in &tails {
                    let key = (tail.body.clone(), tail.instance.clone(), tail.port);
                    let Some(value) = emitted.get(&key).cloned() else {
                        continue;
                    };
                    for head in &heads {
                        let slot = (head.body.clone(), head.instance.clone(), head.port);
                        if !taken.insert(slot) {
                            reports.push(UniverseReport::Link(LinkRefusal {
                                link: link.id.clone(),
                                body: head.body.clone(),
                                member: Address {
                                    instance: head.instance.clone(),
                                    port: head.port,
                                },
                                kind: LinkRefusalKind::NotDelivered,
                            }));
                            continue;
                        }
                        if link.order == Order::Ordered {
                            let held = matches!(
                                check_capability(&self.runtime, &link.id, &head.body),
                                Verdict::Ok(())
                            );
                            if !held {
                                reports.push(UniverseReport::Link(LinkRefusal {
                                    link: link.id.clone(),
                                    body: head.body.clone(),
                                    member: Address {
                                        instance: head.instance.clone(),
                                        port: head.port,
                                    },
                                    kind: LinkRefusalKind::CapabilityNotHeld,
                                }));
                                continue;
                            }
                        }
                        let addr = Address {
                            instance: head.instance.clone(),
                            port: head.port,
                        };
                        match self.inject(&head.body, &addr, value.clone(), self.spent) {
                            Verdict::Ok(()) => {
                                changed = true;
                                pending.insert(head.body.clone(), (link.id.clone(), addr));
                            }
                            Verdict::Refused(_) => {
                                reports.push(UniverseReport::Link(LinkRefusal {
                                    link: link.id.clone(),
                                    body: head.body.clone(),
                                    member: addr,
                                    kind: LinkRefusalKind::Refused,
                                }));
                            }
                        }
                    }
                }
            }
            if !changed {
                return Verdict::Ok(reports);
            }
        }
    }
}
