//! Deliver emitted tail values across hyperedges in link-id order.

use joinn_frame::{Value, Verdict};
use std::collections::{BTreeMap, BTreeSet};

use crate::Address;
use crate::capability::check_capability;
use crate::universe::{Mark, Order};
use crate::universe_state::{LinkRefusal, LinkRefusalKind, UniverseReport, UniverseState};

pub(in crate::universe_state::run) fn deliver_links(
    state: &mut UniverseState,
    emitted: &BTreeMap<(String, String, u32), Value>,
    changed: &mut bool,
    reports: &mut Vec<UniverseReport>,
    delivered_this_pass: &mut BTreeMap<(String, String, u32), (String, Address)>,
) -> bool {
    let mut linked = false;
    let mut links = state.universe.coding.links.clone();
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
                        check_capability(&state.runtime, &link.id, &head.body),
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
                match state.inject(&head.body, &addr, value.clone(), state.spent) {
                    Verdict::Ok(()) => {
                        *changed = true;
                        linked = true;
                        delivered_this_pass.insert(
                            (head.body.clone(), head.instance.clone(), head.port),
                            (link.id.clone(), addr),
                        );
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
    linked
}
