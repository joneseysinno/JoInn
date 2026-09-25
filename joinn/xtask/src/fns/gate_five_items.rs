//! Phase 5 gate table. Eight items. The old path and artifact rows are the harness now.

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g5_assemble, g5_assemble_control, g5_exclusive, g5_exclusive_control, g5_law4, g5_law4_control,
    g5_lenses, g5_lenses_control, g5_linked, g5_linked_control, g5_locality, g5_locality_control,
    g5_membrane, g5_membrane_control, g5_revoke, g5_revoke_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Something crosses",
        check: g5_linked,
        control: g5_linked_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::DropLink("e0"),
    },
    GateItem {
        name: "The membrane is measured",
        check: g5_membrane,
        control: g5_membrane_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: Mutation::DropWire("cli_a@1", "sum@0"),
    },
    GateItem {
        name: "The universe is well-formed",
        check: g5_assemble,
        control: g5_assemble_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::ShiftPort("e0", "calc.sum@2", 9),
    },
    GateItem {
        name: "Exclusivity holds",
        check: g5_exclusive,
        control: g5_exclusive_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::CopyMember("function", "units", "calculation"),
    },
    GateItem {
        name: "Two lenses, one body",
        check: g5_lenses,
        control: g5_lenses_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::DropLens("deployment"),
    },
    GateItem {
        name: "Law 4 is a check",
        check: g5_law4,
        control: g5_law4_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::WireAcross("e0"),
    },
    GateItem {
        name: "A capability can be revoked",
        check: g5_revoke,
        control: g5_revoke_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::DropLink("path"),
    },
    GateItem {
        name: "A refusal stays home",
        check: g5_locality,
        control: g5_locality_control,
        control_artifact: "corpus/phase5/controls/inner_reason.txt",
        opposes: Mutation::Replace("e0"),
    },
];

pub(crate) fn gate_five_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
