//! Rebuild universe coding from CSR arrays.

use crate::universe::{BodyBinding, Link, Member, UniverseCoding};

use super::Csr;

/// Rebuild the hashed coding region from CSR. Round-trips with `print_universe`.
pub fn coding_from_csr(csr: &Csr) -> UniverseCoding {
    let mut bodies = Vec::new();
    for (alias, hash) in csr.body_aliases.iter().zip(csr.body_hashes.iter()) {
        bodies.push(BodyBinding {
            hash: *hash,
            alias: alias.clone(),
        });
    }
    let mut links = Vec::new();
    for (i, id) in csr.link_ids.iter().enumerate() {
        let start = csr.link_offsets[i] as usize;
        let end = csr.link_offsets[i + 1] as usize;
        let mut members = Vec::new();
        for m in &csr.members[start..end] {
            members.push(Member {
                body: csr.body_aliases[m.body as usize].clone(),
                instance: csr.instances[m.instance as usize].clone(),
                port: m.port,
                mark: m.mark,
            });
        }
        links.push(Link {
            id: id.clone(),
            order: csr.link_orders[i],
            members,
        });
    }
    UniverseCoding {
        codex: csr.codex,
        bodies,
        links,
        cross_wires: Vec::new(),
        lenses: csr.lenses.clone(),
    }
}
