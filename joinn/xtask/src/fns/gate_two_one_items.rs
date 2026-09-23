//! Phase 2.1 gate table.

use super::{
    p21_agree, p21_agree_control, p21_findings, p21_findings_control, p21_pairing,
    p21_pairing_control, p21_trace, p21_trace_control, p21_transcript, p21_transcript_control,
    p21_turn, p21_turn_control, p21_v33, p21_v33_control, p21_wrapping, p21_wrapping_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "references agree; a blind seal fails the item",
        check: p21_agree,
        control: p21_agree_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "V33 refuses mutant 14",
        check: p21_v33,
        control: p21_v33_control,
        control_artifact: "corpus/phase22/counterfeit/int_add.body",
    },
    GateItem {
        name: "floor pairing",
        check: p21_pairing,
        control: p21_pairing_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "agree catches wrapping",
        check: p21_wrapping,
        control: p21_wrapping_control,
        control_artifact: "corpus/phase22/counterfeit/int_add.body",
    },
    GateItem {
        name: "unwitnessed turn refused; turn 1 admits",
        check: p21_turn,
        control: p21_turn_control,
        control_artifact: "corpus/phase21/mul.cell",
    },
    GateItem {
        name: "calculator.trace replays",
        check: p21_trace,
        control: p21_trace_control,
        control_artifact: "corpus/transcripts/calculator.trace",
    },
    GateItem {
        name: "stdin session matches transcript",
        check: p21_transcript,
        control: p21_transcript_control,
        control_artifact: "corpus/transcripts/calculator.txt",
    },
    GateItem {
        name: "xtask does not write Findings",
        check: p21_findings,
        control: p21_findings_control,
        control_artifact: "xtask/src/fns/xtask_writes_findings.rs",
    },
];

pub(crate) fn gate_two_one_items() -> &'static [GateItem] {
    ITEMS
}
