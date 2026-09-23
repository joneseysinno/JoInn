//! Build CSR arrays from a universe coding region.

use std::collections::BTreeMap;

use super::{Csr, CsrMember};
use crate::universe::UniverseCoding;

/// Derive CSR incidence from a universe. The arrays are a representation, not an identity.
pub fn csr_from_universe(coding: &UniverseCoding) -> Csr {
    let mut body_aliases = Vec::new();
    let mut body_hashes = Vec::new();
    let mut body_index = BTreeMap::new();
    for body in &coding.bodies {
        let i = body_aliases.len() as u16;
        body_index.insert(body.alias.clone(), i);
        body_aliases.push(body.alias.clone());
        body_hashes.push(body.hash);
    }
    let mut instances = Vec::new();
    let mut instance_index = BTreeMap::new();
    let mut link_ids = Vec::new();
    let mut link_orders = Vec::new();
    let mut link_offsets = Vec::new();
    let mut members = Vec::new();
    for link in &coding.links {
        link_offsets.push(members.len() as u32);
        link_ids.push(link.id.clone());
        link_orders.push(link.order);
        for member in &link.members {
            let body = match body_index.get(&member.body) {
                Some(i) => *i,
                None => continue,
            };
            let instance = match instance_index.get(&member.instance) {
                Some(i) => *i,
                None => {
                    let i = instances.len() as u16;
                    instance_index.insert(member.instance.clone(), i);
                    instances.push(member.instance.clone());
                    i
                }
            };
            members.push(CsrMember {
                body,
                instance,
                port: member.port,
                mark: member.mark,
            });
        }
    }
    link_offsets.push(members.len() as u32);
    Csr {
        codex: coding.codex,
        body_aliases,
        body_hashes,
        instances,
        link_ids,
        link_orders,
        link_offsets,
        members,
        lenses: coding.lenses.clone(),
    }
}
