//! Phase 3 gate table.

use super::{
    g3_artifacts, g3_artifacts_control, g3_descriptions, g3_descriptions_control, g3_hosts,
    g3_hosts_control, g3_intent, g3_intent_control, g3_lock, g3_lock_control, g3_run_gone,
    g3_run_gone_control, g3_signals, g3_signals_control, g3_transcript, g3_transcript_control,
    g3_value, g3_value_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "One body, two hosts",
        check: g3_hosts,
        control: g3_hosts_control,
        control_artifact: "corpus/phase3/columns_reader.body",
        opposes: (),
    },
    GateItem {
        name: "The descriptions agree",
        check: g3_descriptions,
        control: g3_descriptions_control,
        control_artifact: "corpus/phase3/controls/planted_desc.desc",
        opposes: (),
    },
    GateItem {
        name: "A description is a value",
        check: g3_value,
        control: g3_value_control,
        control_artifact: "crates/joinn-host/host_fixtures/plant_render.rs",
        opposes: (),
    },
    GateItem {
        name: "The transcript survived the move",
        check: g3_transcript,
        control: g3_transcript_control,
        control_artifact: "corpus/phase3/controls/no_indent_transcript.txt",
        opposes: (),
    },
    GateItem {
        name: "The intent set is the body's",
        check: g3_intent,
        control: g3_intent_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: (),
    },
    GateItem {
        name: "Signals are declared",
        check: g3_signals,
        control: g3_signals_control,
        control_artifact: "corpus/phase3/columns_reader.body",
        opposes: (),
    },
    GateItem {
        name: "joinn-run is gone",
        check: g3_run_gone,
        control: g3_run_gone_control,
        control_artifact: "Cargo.toml",
        opposes: (),
    },
    GateItem {
        name: "Every control is an artifact",
        check: g3_artifacts,
        control: g3_artifacts_control,
        control_artifact: "xtask/gate_fixtures/unresolvable.txt",
        opposes: (),
    },
    GateItem {
        name: "The path of truth",
        check: g3_lock,
        control: g3_lock_control,
        control_artifact: "corpus/phase3/controls/bad.lock",
        opposes: (),
    },
];

pub(crate) fn gate_three_items() -> &'static [GateItem] {
    ITEMS
}

#[cfg(test)]
mod tests {
    use super::gate_three_items;

    #[test]
    fn no_two_items_share_a_check() {
        let items = gate_three_items();
        for i in 0..items.len() {
            for j in (i + 1)..items.len() {
                assert!(
                    !std::ptr::fn_addr_eq(items[i].check, items[j].check),
                    "{} and {} share a check",
                    items[i].name,
                    items[j].name
                );
            }
        }
    }
}
