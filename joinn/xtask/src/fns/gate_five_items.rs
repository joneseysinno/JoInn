//! Phase 5 gate table. Eight items. The old path and artifact rows are the harness now.

use super::{
    g5_assemble, g5_assemble_control, g5_exclusive, g5_exclusive_control, g5_law4,
    g5_law4_control, g5_lenses, g5_lenses_control, g5_linked, g5_linked_control, g5_locality,
    g5_locality_control, g5_membrane, g5_membrane_control, g5_revoke, g5_revoke_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "Something crosses",
        check: g5_linked,
        control: g5_linked_control,
        control_artifact: "corpus/phase51/controls/unlinked.universe",
    },
    GateItem {
        name: "The membrane is measured",
        check: g5_membrane,
        control: g5_membrane_control,
        control_artifact: "corpus/phase2/calculator.body",
    },
    GateItem {
        name: "The universe is well-formed",
        check: g5_assemble,
        control: g5_assemble_control,
        control_artifact: "corpus/phase5/controls/no_such_port.universe",
    },
    GateItem {
        name: "Exclusivity holds",
        check: g5_exclusive,
        control: g5_exclusive_control,
        control_artifact: "corpus/phase5/controls/two_systems.universe",
    },
    GateItem {
        name: "Two lenses, one body",
        check: g5_lenses,
        control: g5_lenses_control,
        control_artifact: "corpus/phase5/controls/two_systems.universe",
    },
    GateItem {
        name: "Law 4 is a check",
        check: g5_law4,
        control: g5_law4_control,
        control_artifact: "corpus/phase5/controls/wrong_container.universe",
    },
    GateItem {
        name: "A capability can be revoked",
        check: g5_revoke,
        control: g5_revoke_control,
        control_artifact: "corpus/phase5/universe.universe",
    },
    GateItem {
        name: "A refusal stays home",
        check: g5_locality,
        control: g5_locality_control,
        control_artifact: "corpus/phase5/controls/inner_reason.txt",
    },
];

pub(crate) fn gate_five_items() -> &'static [GateItem] {
    ITEMS
}
