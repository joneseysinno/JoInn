//! Phase 1 gate table.

use super::{g1_demo1, g1_demo1_control, g1_demo2, g1_demo2_control, g1_demo3, g1_demo3_control, g1_demo4, g1_demo4_control};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "add@ℚ admitted; hash unchanged",
        check: g1_demo1,
        control: g1_demo1_control,
        control_artifact: "corpus/phase0/sum.cell",
        opposes: (),
    },
    GateItem {
        name: "commutativity breaker refused",
        check: g1_demo2,
        control: g1_demo2_control,
        control_artifact: "corpus/phase0/sum.cell",
        opposes: (),
    },
    GateItem {
        name: "regulatory edit leaves the hash",
        check: g1_demo3,
        control: g1_demo3_control,
        control_artifact: "corpus/phase0/sum.cell",
        opposes: (),
    },
    GateItem {
        name: "added in-port refused without lineage",
        check: g1_demo4,
        control: g1_demo4_control,
        control_artifact: "corpus/phase0/sum.cell",
        opposes: (),
    },
];

pub(crate) fn gate_one_items() -> &'static [GateItem] {
    ITEMS
}
