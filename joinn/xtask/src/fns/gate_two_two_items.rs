//! Phase 2.2 gate table.

use super::{
    p22_bound, p22_bound_control, p22_corpus, p22_corpus_control, p22_lookup, p22_lookup_control,
    p22_path, p22_path_control, p22_register, p22_register_control, p22_roundtrip,
    p22_roundtrip_control, p22_see, p22_see_control, p22_true, p22_true_control, p22_v33,
    p22_v33_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "every seal can see",
        check: p22_see,
        control: p22_see_control,
        control_artifact: "corpus/phase22/counterfeit/int_add.body",
    },
    GateItem {
        name: "the references are true",
        check: p22_true,
        control: p22_true_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "a bound cannot be empty",
        check: p22_bound,
        control: p22_bound_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "V33 walks the whole genome",
        check: p22_v33,
        control: p22_v33_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "registers are enforced",
        check: p22_register,
        control: p22_register_control,
        control_artifact: "corpus/phase22/int_format_ref.body",
    },
    GateItem {
        name: "the corpus is true",
        check: p22_corpus,
        control: p22_corpus_control,
        control_artifact: "corpus/phase22/counterfeit/false_law.cell",
    },
    GateItem {
        name: "a seal names its own cell",
        check: p22_lookup,
        control: p22_lookup_control,
        control_artifact: "corpus/phase21/mul.cell",
    },
    GateItem {
        name: "the round-trip holds",
        check: p22_roundtrip,
        control: p22_roundtrip_control,
        control_artifact: "corpus/phase22/text_parse_ref.body",
    },
    GateItem {
        name: "the path of truth",
        check: p22_path,
        control: p22_path_control,
        control_artifact: "corpus/phase3/controls/bad.lock",
    },
];

pub(crate) fn gate_two_two_items() -> &'static [GateItem] {
    ITEMS
}
