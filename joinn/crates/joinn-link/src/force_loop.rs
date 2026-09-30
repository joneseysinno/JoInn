//! The first loop among a contact's forces, if any.

use joinn_dna::Force;
use std::collections::{BTreeMap, BTreeSet};

/// Force F reaches force G when one of F's members is G's response. Forces are
/// visited in name order, and reached forces are followed in name order; the
/// first loop found is returned as its path, first name repeated at the end.
pub(crate) fn force_loop(forces: &[Force]) -> Option<Vec<String>> {
    let names: BTreeSet<&str> = forces.iter().map(|f| f.name.as_str()).collect();
    let mut reaches: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for force in forces {
        let entry = reaches.entry(force.name.as_str()).or_default();
        for member in &force.members {
            if names.contains(member.instance.as_str()) {
                entry.insert(member.instance.as_str());
            }
        }
    }
    let mut done: BTreeSet<&str> = BTreeSet::new();
    for start in &names {
        if done.contains(start) {
            continue;
        }
        let mut path: Vec<&str> = vec![start];
        let mut next: Vec<usize> = vec![0];
        while let (Some(&node), Some(i)) = (path.last(), next.last_mut()) {
            let child = reaches.get(node).and_then(|set| set.iter().nth(*i));
            let Some(&child) = child else {
                done.insert(node);
                path.pop();
                next.pop();
                continue;
            };
            *i += 1;
            if let Some(pos) = path.iter().position(|n| *n == child) {
                let mut found: Vec<String> = path[pos..].iter().map(|n| (*n).to_owned()).collect();
                found.push(child.to_owned());
                return Some(found);
            }
            if !done.contains(child) {
                path.push(child);
                next.push(0);
            }
        }
    }
    None
}
