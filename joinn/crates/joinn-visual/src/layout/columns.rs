//! The column of each instance: the length of the longest wire path that ends at it.

use joinn_dna::Wire;
use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use crate::refuse::refuse;

/// Instances with no incoming wire are column 0. A wire cycle is refused naming
/// the instances it runs through, in name order.
pub(super) fn columns(names: &BTreeSet<String>, wires: &[Wire]) -> Verdict<BTreeMap<String, i64>> {
    let mut incoming: BTreeMap<&str, usize> = names.iter().map(|n| (n.as_str(), 0)).collect();
    let mut outgoing: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for w in wires {
        if let Some(n) = incoming.get_mut(w.dst_instance.as_str()) {
            *n += 1;
        }
        outgoing
            .entry(w.src_instance.as_str())
            .or_default()
            .push(w.dst_instance.as_str());
    }
    let mut depth: BTreeMap<&str, i64> = names.iter().map(|n| (n.as_str(), 0)).collect();
    let mut ready: BTreeSet<&str> = incoming
        .iter()
        .filter(|(_, n)| **n == 0)
        .map(|(name, _)| *name)
        .collect();
    let mut column = BTreeMap::new();
    while let Some(name) = ready.pop_first() {
        let here = depth.get(name).copied().unwrap_or(0);
        column.insert(name.to_owned(), here);
        for dst in outgoing.get(name).into_iter().flatten() {
            if let Some(d) = depth.get_mut(dst) {
                *d = (*d).max(here + 1);
            }
            if let Some(n) = incoming.get_mut(dst) {
                if *n > 0 {
                    *n -= 1;
                }
                if *n == 0 {
                    ready.insert(*dst);
                }
            }
        }
    }
    if column.len() == names.len() {
        return Verdict::Ok(column);
    }
    let mut stuck: BTreeSet<&str> = names
        .iter()
        .map(String::as_str)
        .filter(|n| !column.contains_key(*n))
        .collect();
    loop {
        let sinks: Vec<&str> = stuck
            .iter()
            .copied()
            .filter(|n| {
                !outgoing
                    .get(n)
                    .into_iter()
                    .flatten()
                    .any(|d| stuck.contains(d))
            })
            .collect();
        if sinks.is_empty() {
            break;
        }
        for s in sinks {
            stuck.remove(s);
        }
    }
    let through: Vec<&str> = stuck.into_iter().collect();
    refuse(format!(
        "layout: wire cycle through {}; acceptance is a body whose wires form no cycle",
        through.join(", ")
    ))
}
