//! Activation stack frame: instances, mail queue, grants.

mod build;
mod fan_boundary;

use joinn_dna::{Body, Cell};
use joinn_frame::Value;
use std::collections::{BTreeMap, BTreeSet};

use crate::slot::Slot;

pub(crate) use build::build_activation;
pub(crate) use fan_boundary::fan_boundary;

#[derive(Clone, Debug)]
pub(crate) struct Mail {
    pub(crate) dest: String,
    pub(crate) port: u32,
    pub(crate) value: Value,
    pub(crate) grant_epoch: u64,
    pub(crate) wire_position: u32,
    pub(crate) message_sequence: u64,
}

pub(crate) struct Instance {
    pub(crate) cell: Cell,
    pub(crate) slots: BTreeMap<u32, Slot>,
    pub(crate) is_prim: bool,
}

pub(crate) type Key = (u64, u32, u64, u64, String, u32);

pub(crate) struct Activation {
    pub(crate) body: Body,
    pub(crate) instances: BTreeMap<String, Instance>,
    pub(crate) queue: BTreeSet<Key>,
    pub(crate) mail: BTreeMap<Key, Mail>,
    pub(crate) return_to: Option<String>,
    pub(crate) boundary: Option<String>,
    pub(crate) label: String,
    pub(crate) grant_holder: BTreeMap<String, String>,
    pub(crate) nested: bool,
}
