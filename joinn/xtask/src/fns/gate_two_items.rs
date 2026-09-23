//! Phase 2 gate table.

use super::{
    g2_agree, g2_agree_control, g2_alleles, g2_alleles_control, g2_bodies, g2_bodies_control,
    g2_corpus, g2_corpus_control, g2_evolution, g2_evolution_control, g2_trace, g2_trace_control,
    g2_transcript, g2_transcript_control, g2_turn, g2_turn_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "the corpus verifies",
        check: g2_corpus,
        control: g2_corpus_control,
        control_artifact: "corpus/phase22/counterfeit/false_law.cell",
    },
    GateItem {
        name: "references agree",
        check: g2_agree,
        control: g2_agree_control,
        control_artifact: "corpus/phase21/int_add_ref.body",
    },
    GateItem {
        name: "the transcript matches",
        check: g2_transcript,
        control: g2_transcript_control,
        control_artifact: "corpus/transcripts/calculator.txt",
    },
    GateItem {
        name: "the trace replays",
        check: g2_trace,
        control: g2_trace_control,
        control_artifact: "corpus/transcripts/calculator.trace",
    },
    GateItem {
        name: "the turn produces 2",
        check: g2_turn,
        control: g2_turn_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
    },
    GateItem {
        name: "sum_turn is admitted",
        check: g2_evolution,
        control: g2_evolution_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
    },
    GateItem {
        name: "phase 2 bodies share a coding region",
        check: g2_bodies,
        control: g2_bodies_control,
        control_artifact: "corpus/phase2/calculator.body",
    },
    GateItem {
        name: "hand-written turn alleles",
        check: g2_alleles,
        control: g2_alleles_control,
        control_artifact: "corpus/phase2/sum_turn.cell",
    },
];

pub(crate) fn gate_two_items() -> &'static [GateItem] {
    ITEMS
}
