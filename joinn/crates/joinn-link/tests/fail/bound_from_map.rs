fn main() {
    let map: std::collections::BTreeMap<
        String,
        (
            joinn_dna::Body,
            std::collections::BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
        ),
    > = std::collections::BTreeMap::new();
    let _ = joinn_link::Bound { by_alias: map };
}
