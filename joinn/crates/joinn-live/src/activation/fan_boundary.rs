//! Fan boundary inputs into nested activation slots and wires.

use joinn_dna::JoinPolicy;
use joinn_frame::Value;
use std::collections::BTreeMap;

use crate::activation::{Activation, Mail};
use crate::slot::{SlotWrite, apply_slot};

pub(crate) fn fan_boundary(
    act: &mut Activation,
    boundary: &str,
    inputs: &BTreeMap<u32, Value>,
    seq: &mut u64,
) {
    let policy = act
        .instances
        .get(boundary)
        .map(|i| i.cell.coding.contract.join_policy)
        .unwrap_or(JoinPolicy::Latest);
    if let Some(inst) = act.instances.get_mut(boundary) {
        for (port, val) in inputs {
            let _ = apply_slot(
                inst,
                *port,
                SlotWrite::Fill(val.clone()),
                policy,
                boundary,
                0,
            );
        }
    }
    let wires = act.body.coding.wires.clone();
    let mut wire_pos = 0u32;
    for w in wires {
        if w.src_instance != boundary {
            continue;
        }
        if let Some(val) = inputs.get(&w.src_port) {
            let s = *seq;
            *seq += 1;
            let m = Mail {
                dest: w.dst_instance.clone(),
                port: w.dst_port,
                value: val.clone(),
                grant_epoch: 0,
                wire_position: wire_pos,
                message_sequence: s,
            };
            wire_pos += 1;
            let key = (
                m.grant_epoch,
                m.wire_position,
                m.message_sequence,
                s,
                m.dest.clone(),
                m.port,
            );
            act.queue.insert(key.clone());
            act.mail.insert(key, m);
        }
    }
}
