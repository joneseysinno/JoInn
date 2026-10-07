//! The phase gates in lock order: each one's argument, label, runner and table.

use joinn_gate::GateItem;

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    PHASE_LABELS, gate_five, gate_five_items, gate_five_one, gate_five_one_items, gate_five_two,
    gate_five_two_items, gate_four, gate_four_items, gate_one, gate_one_items, gate_seven,
    gate_seven_four, gate_seven_four_items, gate_seven_items, gate_seven_three,
    gate_seven_three_items, gate_seven_two, gate_seven_two_items, gate_six, gate_six_items,
    gate_three, gate_three_items, gate_two, gate_two_items, gate_two_one, gate_two_one_items,
    gate_two_two, gate_two_two_items,
};

/// A legacy table's checks must pass and its controls are not graded; an
/// opposed table's controls are graded before its checks run.
#[derive(Clone, Copy)]
pub(crate) enum GateTable {
    Legacy(&'static [GateItem]),
    Opposed(&'static [GateItem<Subject, Mutation>]),
}

/// A phase gate's runner: prints its rows and returns its score.
pub(crate) type GateRun = fn(&str) -> Result<(u32, u32), String>;

/// One phase gate: `cargo xtask gate <arg>`.
#[derive(Clone, Copy)]
pub(crate) struct PhaseGate {
    pub(crate) arg: &'static str,
    pub(crate) label: &'static str,
    pub(crate) run: GateRun,
    pub(crate) table: GateTable,
}

/// Phases 1 onward, in `PHASE_LABELS` order (phase 0 is the corpus).
pub(crate) fn phase_gates() -> Vec<PhaseGate> {
    use GateTable::{Legacy, Opposed};
    let rows: [(&str, GateRun, GateTable); 14] = [
        ("1", gate_one, Legacy(gate_one_items())),
        ("2", gate_two, Legacy(gate_two_items())),
        ("2.1", gate_two_one, Legacy(gate_two_one_items())),
        ("2.2", gate_two_two, Legacy(gate_two_two_items())),
        ("3", gate_three, Legacy(gate_three_items())),
        ("4", gate_four, Opposed(gate_four_items())),
        ("5", gate_five, Opposed(gate_five_items())),
        ("5.1", gate_five_one, Opposed(gate_five_one_items())),
        ("5.2", gate_five_two, Opposed(gate_five_two_items())),
        ("6", gate_six, Opposed(gate_six_items())),
        ("7", gate_seven, Opposed(gate_seven_items())),
        ("7.2", gate_seven_two, Opposed(gate_seven_two_items())),
        ("7.3", gate_seven_three, Opposed(gate_seven_three_items())),
        ("7.4", gate_seven_four, Opposed(gate_seven_four_items())),
    ];
    rows.into_iter()
        .zip(PHASE_LABELS.iter().skip(1))
        .map(|((arg, run, table), label)| PhaseGate {
            arg,
            label,
            run,
            table,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::phase_gates;
    use crate::fns::PHASE_LABELS;

    #[test]
    fn every_label_after_phase_0_has_one_gate_and_its_argument_is_the_label() {
        let gates = phase_gates();
        assert_eq!(gates.len(), PHASE_LABELS.len() - 1);
        for gate in &gates {
            assert_eq!(gate.label.strip_prefix("phase "), Some(gate.arg));
        }
    }
}
