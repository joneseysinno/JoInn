//! Read a live instance into a description. Never writes.

use joinn_dna::{Direction, hash};
use joinn_frame::{CheckId, Refusal, Verdict};

use super::{Description, PortFace};
use crate::role::Role;
use joinn_live::BodyState;

/// Read a live instance into a description. Never writes.
pub fn describe(state: &BodyState, instance: &str) -> Verdict<Description> {
    let Some(cell) = state.instance_cell(instance) else {
        return Verdict::Refused(Refusal::structural(
            CheckId::Other,
            format!("describe: unknown instance {instance}"),
        ));
    };
    let Some(body) = state.root_body() else {
        return Verdict::Refused(Refusal::structural(CheckId::Other, "describe: empty stack"));
    };
    let label = body
        .regulatory
        .labels
        .get(instance)
        .cloned()
        .or_else(|| body.regulatory.names.get(instance).cloned())
        .unwrap_or_else(|| instance.to_owned());
    let last = state.last_ports(instance);
    let mut ports = Vec::new();
    let mut decls = cell.coding.contract.ports.clone();
    decls.sort_by_key(|p| p.position);
    for p in decls {
        let name = cell
            .regulatory
            .names
            .get(&p.position)
            .cloned()
            .unwrap_or_default();
        let port_label = cell
            .regulatory
            .labels
            .get(&p.position)
            .cloned()
            .unwrap_or_else(|| name.clone());
        let role = match p.direction {
            Direction::In => Role::Input,
            Direction::Out => Role::Output,
        };
        let value = last
            .and_then(|m| m.get(&p.position))
            .or_else(|| state.slot(instance, p.position))
            .map(joinn_frame::Value::print_term);
        ports.push(PortFace {
            position: p.position,
            direction: p.direction,
            frame: p.frame,
            value,
            name,
            label: port_label,
            role,
        });
    }
    Verdict::Ok(Description {
        cell: hash(&cell.coding),
        instance: instance.to_owned(),
        ports,
        label,
        role: Role::Cell,
    })
}
