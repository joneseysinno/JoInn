//! Live state of one body. Nesting is a stack, never a Rust call.

mod activation_path;
mod advance_grants;
mod budget_refusal;
mod consume_and_emit;
mod deliver;
mod depth;
mod enqueue;
mod enqueue_outs;
mod finish_fire;
mod inject;
mod new;
mod next_seq;
mod pop_nested;
mod push_dna;
mod run;
mod set_budget;
mod step;
mod try_fire;

#[cfg(test)]
pub(crate) mod fixtures;

use joinn_dna::{Body, Cell};
use joinn_frame::{FrameRegistry, Hash, Value};
use joinn_gate::NativeRegistry;
use std::collections::BTreeMap;

use crate::activation::Activation;
use crate::slot::Slot;

/// Live state of one body. Nesting is a stack, never a Rust call.
pub struct BodyState {
    stack: Vec<Activation>,
    natives: NativeRegistry,
    frames: FrameRegistry,
    cells: BTreeMap<Hash, Cell>,
    bodies: BTreeMap<Hash, Body>,
    step: u64,
    seq: u64,
    seed: u64,
    last_mail: Option<String>,
    /// Delivery path counter (V35).
    deliveries: u64,
    /// Last fired port map per instance (ins at fire + outs produced).
    last_ports: BTreeMap<String, BTreeMap<u32, Value>>,
    budget: u64,
    last_refusal: Option<String>,
    /// Instance and port for the delivery that led to the last refusal.
    refusal_site: Option<(String, u32)>,
}

impl BodyState {
    /// Bodies the engine may run as `AlleleBody::Dna`.
    pub fn with_bodies(mut self, bodies: BTreeMap<Hash, Body>) -> Self {
        self.bodies = bodies;
        self
    }

    /// How many times `deliver` ran. V35: there is one path.
    pub fn delivery_count(&self) -> u64 {
        self.deliveries
    }

    /// Read an instance slot on the root activation.
    pub fn slot(&self, instance: &str, port: u32) -> Option<&Value> {
        let inst = self.stack.first()?.instances.get(instance)?;
        match inst.slots.get(&port)? {
            Slot::Filled(v) => Some(v),
            Slot::Queue(q) => q.first(),
            Slot::Empty => None,
        }
    }

    /// Ports present at the last fire of `instance`.
    pub fn last_ports(&self, instance: &str) -> Option<&BTreeMap<u32, Value>> {
        self.last_ports.get(instance)
    }

    /// Step count consumed so far.
    pub fn steps(&self) -> u64 {
        self.step
    }

    /// The root body's genome and regulatory.
    pub fn root_body(&self) -> Option<&Body> {
        self.stack.first().map(|a| &a.body)
    }

    /// The cell bound to a root instance.
    pub fn instance_cell(&self, instance: &str) -> Option<&Cell> {
        Some(&self.stack.first()?.instances.get(instance)?.cell)
    }

    /// The refusing body's own words, if the last `run` refused.
    pub fn last_refusal(&self) -> Option<&str> {
        self.last_refusal.as_deref()
    }

    /// Membrane site for the last refusal, if any.
    pub fn refusal_site(&self) -> Option<(&str, u32)> {
        self.refusal_site
            .as_ref()
            .map(|(inst, port)| (inst.as_str(), *port))
    }
}
