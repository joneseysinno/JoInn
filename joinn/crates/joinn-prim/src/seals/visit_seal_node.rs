//! DFS visit for seal DAG check.

use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use super::refuse::refuse;

pub(in crate::seals) fn visit_seal_node(
    n: &str,
    edges: &BTreeMap<String, Vec<String>>,
    stack: &mut Vec<String>,
    seen: &mut BTreeSet<String>,
) -> Verdict<()> {
    if stack.iter().any(|s| s == n) {
        return Verdict::Refused(refuse(&format!("V42: cyclic seals {}", stack.join(" → "))));
    }
    if !seen.insert(n.to_string()) {
        return Verdict::Ok(());
    }
    stack.push(n.to_string());
    if let Some(succ) = edges.get(n) {
        for s in succ {
            match visit_seal_node(s, edges, stack, seen) {
                Verdict::Ok(()) => {}
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
    }
    stack.pop();
    Verdict::Ok(())
}
