//! No two non-legacy items share (control_artifact, opposes).

use crate::fns::mutate::Mutation;
use crate::fns::opposed_tables::opposed_tables;

/// Refuse when two non-legacy rows declare the same opposition pair.
pub(crate) fn check_distinct_opposition() -> Result<(), String> {
    let mut seen: Vec<(&str, &Mutation, &str)> = Vec::new();
    for (_, table) in opposed_tables() {
        for item in table {
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
