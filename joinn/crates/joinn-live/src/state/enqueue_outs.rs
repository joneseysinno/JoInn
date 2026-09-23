//! Enqueue outgoing wire mail.

use joinn_dna::Wire;
use joinn_frame::Value;
use std::collections::BTreeMap;

use crate::activation::Mail;
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn enqueue_outs(
        &mut self,
        depth: usize,
        name: &str,
        outs: &BTreeMap<u32, Value>,
    ) {
        let wires: Vec<Wire> = self
            .stack
            .get(depth)
            .map(|a| a.body.coding.wires.clone())
            .unwrap_or_default();
        let mut wire_pos = 0u32;
        for w in wires {
            if w.src_instance != name {
                continue;
            }
            if let Some(val) = outs.get(&w.src_port) {
                let seq = self.next_seq();
                let m = Mail {
                    dest: w.dst_instance.clone(),
                    port: w.dst_port,
                    value: val.clone(),
                    grant_epoch: 0,
                    wire_position: wire_pos,
                    message_sequence: seq,
                };
                wire_pos += 1;
                self.enqueue(depth, m);
            }
        }
    }
}
