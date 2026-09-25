//! No two non-legacy items share (control_artifact, opposes).

use crate::fns::gate_five_items::gate_five_items;
use crate::fns::gate_five_one_items::gate_five_one_items;
use crate::fns::gate_five_two_items::gate_five_two_items;
use crate::fns::mutate::Mutation;
use crate::fns::subject::Subject;
use joinn_gate::GateItem;

/// Refuse when two non-legacy rows declare the same opposition pair.
pub(crate) fn check_distinct_opposition() -> Result<(), String> {
    let tables: &[&[GateItem<Subject, Mutation>]] = &[
        gate_five_items(),
        gate_five_one_items(),
        gate_five_two_items(),
    ];
    let mut seen: Vec<(&str, &Mutation, &str)> = Vec::new();
    for table in tables {
        for item in *table {
            for (art, opp, name) in &seen {
                if *art == item.control_artifact && *opp == &item.opposes {
                    return Err(format!(
                        "distinct opposition: {name} and {} share ({}, {:?})",
                        item.name, item.control_artifact, item.opposes
                    ));
                }
            }
            seen.push((item.control_artifact, &item.opposes, item.name));
        }
    }
    Ok(())
}
