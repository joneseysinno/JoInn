//! Acyclic seal graph.

use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use super::visit_seal_node::visit_seal_node;

/// Acyclic seal graph. A pair that names each other refuses.
pub fn check_seal_dag(edges: &BTreeMap<String, Vec<String>>) -> Verdict<()> {
    let mut seen = BTreeSet::new();
    for k in edges.keys() {
        match visit_seal_node(k, edges, &mut Vec::new(), &mut seen) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_seal_dag;
    use joinn_frame::Verdict;
    use std::collections::BTreeMap;

    #[test]
    fn cyclic_seals_refuse() {
        let mut e = BTreeMap::new();
        e.insert("a".into(), vec!["b".into()]);
        e.insert("b".into(), vec!["a".into()]);
        match check_seal_dag(&e) {
            Verdict::Refused(r) => assert!(
                r.reason.contains("cyclic") || r.reason.contains("V42"),
                "{}",
                r.reason
            ),
            Verdict::Ok(()) => panic!("cycle must refuse"),
        }
    }
}
