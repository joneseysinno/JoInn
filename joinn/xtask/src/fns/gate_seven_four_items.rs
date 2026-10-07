//! Gate 7.4 table (P74-14: a body grows by its DNA, evolution keeps its
//! witnesses and gains, a force is seen as a lasso).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g74_evolution, g74_evolution_control, g74_growth, g74_growth_control, g74_lasso,
    g74_lasso_control,
};
use joinn_dna::Accept;
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "A body grows by its DNA, and every size is true",
        check: g74_growth,
        check_name: "g74_growth",
        control: g74_growth_control,
        control_name: "g74_growth_control",
        control_artifact: "corpus/phase74/counting.system",
        opposes: Mutation::Accepts("numbers", Accept::Any),
    },
    GateItem {
        name: "Evolution keeps every old witness, and gains",
        check: g74_evolution,
        check_name: "g74_evolution",
        control: g74_evolution_control,
        control_name: "g74_evolution_control",
        control_artifact: "corpus/phase74/adding.system",
        opposes: Mutation::Accepts("numbers", Accept::One),
    },
    GateItem {
        name: "A force is seen as a lasso; growth is not identity",
        check: g74_lasso,
        check_name: "g74_lasso",
        control: g74_lasso_control,
        control_name: "g74_lasso_control",
        control_artifact: "corpus/phase74/counting.system",
        opposes: Mutation::DropForce("count"),
    },
];

pub(crate) fn gate_seven_four_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
