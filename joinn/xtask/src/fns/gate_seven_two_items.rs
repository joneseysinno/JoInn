//! Gate 7.2 table (P72-14: the exact zoom, bands and two pickers, folded systems).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g72_bands, g72_bands_control, g72_folded, g72_folded_control, g72_zoom, g72_zoom_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "The zoom is exact, and a rebase moves nothing",
        check: g72_zoom,
        control: g72_zoom_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::DropLens("function"),
    },
    GateItem {
        name: "Bands follow size, and two pickers name every pixel",
        check: g72_bands,
        control: g72_bands_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::CopyMember("function", "units", "calculation"),
    },
    GateItem {
        name: "A folded system is one node, and a link touches it once",
        check: g72_folded,
        control: g72_folded_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::ShiftPort("path", "calc.sum@2", 9),
    },
];

pub(crate) fn gate_seven_two_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn only_gate_seven_two_s_items_name_the_grove() {
        let fns = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("fns");
        let mut named = Vec::new();
        let walk = crate::fns::walk_rs(&fns, &mut |path, text| {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                return;
            };
            let digits = name.strip_prefix('g').map_or(0, |rest| {
                rest.chars().take_while(char::is_ascii_digit).count()
            });
            let gate_leaf = digits > 0 && name[1 + digits..].starts_with('_');
            if gate_leaf && text.contains("grove") {
                named.push(name.to_owned());
            }
        });
        assert!(walk.is_ok(), "{}", walk.err().unwrap_or_default());
        named.sort();
        assert_eq!(
            named,
            [
                "g72_bands.rs",
                "g72_folded.rs",
                "g72_layouts.rs",
                "g72_reversible.rs",
                "g72_zoom.rs",
            ]
        );
    }
}
