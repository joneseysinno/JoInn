//! Phase 2 gate table. Four items after §2.4 deletions.

use super::{
    g2_alleles, g2_alleles_control, g2_bodies, g2_bodies_control, g2_evolution,
    g2_evolution_control, g2_turn, g2_turn_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "the turn produces 2",
        check: g2_turn,
        control: g2_turn_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
        opposes: (),
    },
    GateItem {
        name: "sum_turn is admitted",
        check: g2_evolution,
        control: g2_evolution_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
        opposes: (),
    },
    GateItem {
        name: "phase 2 bodies share a coding region",
        check: g2_bodies,
        control: g2_bodies_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: (),
    },
    GateItem {
        name: "hand-written turn alleles",
        check: g2_alleles,
        control: g2_alleles_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
        opposes: (),
    },
];

pub(crate) fn gate_two_items() -> &'static [GateItem] {
    ITEMS
}
