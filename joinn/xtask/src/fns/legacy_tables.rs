//! Every legacy gate table, read from the gate registry.

use joinn_gate::GateItem;

use super::gate_table::{GateTable, phase_gates};

/// Each legacy phase gate's label and table, in lock order.
pub(crate) fn legacy_tables() -> Vec<(&'static str, &'static [GateItem])> {
    phase_gates()
        .into_iter()
        .filter_map(|gate| match gate.table {
            GateTable::Legacy(items) => Some((gate.label, items)),
            GateTable::Opposed(_) => None,
        })
        .collect()
}
