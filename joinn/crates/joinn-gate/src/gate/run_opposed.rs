//! Opposed gate table runner.

use super::GateItem;

/// Run an opposed table. Fails naming the item if a control passes on the real subject.
pub fn run_opposed<S>(
    items: &[GateItem<S>],
    subjects: &[S],
) -> Result<Vec<(usize, &'static str, bool)>, String> {
    if items.len() != subjects.len() {
        return Err(format!(
            "gate table has {} items and {} subjects; acceptance is one subject per item",
            items.len(),
            subjects.len()
        ));
    }
    let mut rows = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let n = i + 1;
        if (item.control)(&subjects[i]) {
            return Err(format!("gate item {n} ({}) control passed", item.name));
        }
        rows.push((n, item.name, (item.check)()));
    }
    Ok(rows)
}
