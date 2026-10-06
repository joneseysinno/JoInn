//! Every opposed gate table, read from the gate registry.

use joinn_gate::GateItem;

use super::gate_table::{GateTable, phase_gates};
use super::mutate::Mutation;
use super::subject::Subject;

/// Each opposed phase gate's label and table, in lock order.
pub(crate) fn opposed_tables() -> Vec<(&'static str, &'static [GateItem<Subject, Mutation>])> {
    phase_gates()
        .into_iter()
        .filter_map(|gate| match gate.table {
            GateTable::Opposed(items) => Some((gate.label, items)),
            GateTable::Legacy(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::opposed_tables;
    use crate::fns::gate_table::phase_gates;
    use crate::fns::legacy_tables::legacy_tables;

    #[test]
    fn cross_gate_tables_are_the_registry_s_tables() {
        let opposed: Vec<&str> = opposed_tables().iter().map(|(l, _)| *l).collect();
        let legacy: Vec<&str> = legacy_tables().iter().map(|(l, _)| *l).collect();
        let registry: Vec<&str> = phase_gates().iter().map(|g| g.label).collect();
        let mut both: Vec<&str> = legacy.iter().chain(opposed.iter()).copied().collect();
        both.sort();
        let mut sorted = registry.clone();
        sorted.sort();
        assert_eq!(both, sorted);
        assert_eq!(
            legacy,
            ["phase 1", "phase 2", "phase 2.1", "phase 2.2", "phase 3"]
        );
        assert!(opposed.contains(&"phase 7.3"), "{opposed:?}");
        assert_eq!(opposed.last(), registry.last());
    }
}
