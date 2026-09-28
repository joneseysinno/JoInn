//! Gate 4 table (P4-13: the assay reads, names, and a declaration refuses).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g4_declared, g4_declared_control, g4_partner, g4_partner_control, g4_sample, g4_sample_control,
    g4_two_things, g4_two_things_control,
};
use joinn_gate::GateItem;

/// `corpus/phase4/cli_input_open.cell`, as recorded in `hashes.txt`.
pub(crate) const CLI_INPUT_OPEN_HASH: &str =
    "eb8479f42622365a12d5eb6ae3f35386d782cb9679144582774c49ea83945678";
/// `corpus/phase4/fmt_twin.body`, as recorded in `hashes.txt`.
pub(crate) const FMT_TWIN_HASH: &str =
    "f1b05fd17e9eeaa284a03a6ea586073b7b328e377c169628fa347cdec239b0cf";

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "The instrument reads a known sample",
        check: g4_sample,
        control: g4_sample_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: Mutation::SwapCell("cli_b", CLI_INPUT_OPEN_HASH),
    },
    GateItem {
        name: "A promise names its partner",
        check: g4_partner,
        control: g4_partner_control,
        control_artifact: "corpus/phase4/loop.universe",
        opposes: Mutation::SwapBinding("fmt", FMT_TWIN_HASH),
    },
    GateItem {
        name: "A body that is two things is named",
        check: g4_two_things,
        control: g4_two_things_control,
        control_artifact: "corpus/phase52/adversary/asker.body",
        opposes: Mutation::AddWire("question@2", "answer@0"),
    },
    GateItem {
        name: "A declaration refuses; an assay doesn't",
        check: g4_declared,
        control: g4_declared_control,
        control_artifact: "corpus/phase4/loop_declared.universe",
        opposes: Mutation::SwapBinding("fmt", FMT_TWIN_HASH),
    },
];

pub(crate) fn gate_four_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}

#[cfg(test)]
mod tests {
    use super::{CLI_INPUT_OPEN_HASH, FMT_TWIN_HASH};
    use crate::fns::hashes_path::hashes_path;
    use std::fs;

    #[test]
    fn the_hash_consts_are_the_recorded_goldens() {
        let path = hashes_path().unwrap_or_else(|e| panic!("{e}"));
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let golden = |name: &str| -> String {
            text.lines()
                .find_map(|line| {
                    let mut parts = line.split_whitespace();
                    (parts.next() == Some(name)).then(|| parts.next().unwrap_or("").to_string())
                })
                .unwrap_or_else(|| panic!("{name} is not in hashes.txt"))
        };
        assert_eq!(CLI_INPUT_OPEN_HASH, golden("cli_input_open"));
        assert_eq!(FMT_TWIN_HASH, golden("fmt_twin"));
    }
}
