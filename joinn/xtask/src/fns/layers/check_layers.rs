//! Check manifests against §2.1: every edge outside the table is a refusal line.

use std::collections::{BTreeMap, BTreeSet};

use super::{ABOVE, BELOW, EDGES, EXTERNAL, Manifest};

/// Refusal lines, one per edge, dependency or lints table the plan does not allow.
pub(crate) fn check_layers(manifests: &[Manifest]) -> Vec<String> {
    let mut hits = Vec::new();
    let names: BTreeSet<&str> = manifests.iter().map(|m| m.name.as_str()).collect();
    let edges: BTreeMap<&str, Vec<&str>> = manifests
        .iter()
        .map(|m| {
            let ws = m
                .deps
                .iter()
                .map(String::as_str)
                .filter(|d| names.contains(d) || ABOVE.contains(d) || BELOW.contains(d))
                .collect();
            (m.name.as_str(), ws)
        })
        .collect();

    for m in manifests {
        let name = m.name.as_str();
        let known = ABOVE.contains(&name) || BELOW.contains(&name) || name == "xtask";
        if !known {
            hits.push(format!(
                "layers: {name} is not in the plan's §2.1 table; acceptance is a crate the table names"
            ));
        }
        if let Some((_, allowed)) = EDGES.iter().find(|(c, _)| *c == name) {
            for dep in edges.get(name).into_iter().flatten() {
                if !allowed.contains(dep) {
                    hits.push(format!(
                        "layers: {name} -> {dep} is not an edge of the plan's §2.1 table; acceptance is {name} depending on {}",
                        allowed.join(", ")
                    ));
                }
            }
        }
        if name == "xtask" {
            for dep in edges.get(name).into_iter().flatten() {
                if *dep == "joinn-shell-desktop" {
                    hits.push(format!(
                        "layers: xtask -> {dep} is not an edge of the plan's §2.1 table; acceptance is xtask depending on crates above the boundary and joinn-gpu"
                    ));
                }
            }
        }
        if ABOVE.contains(&name) {
            let mut stack = vec![vec![name]];
            let mut seen = BTreeSet::new();
            while let Some(path) = stack.pop() {
                let Some(last) = path.last().copied() else {
                    continue;
                };
                if !seen.insert(last) {
                    continue;
                }
                if BELOW.contains(&last) {
                    hits.push(format!(
                        "layers: {}: a crate above the renderer boundary reaches {last}; acceptance is no path from above the boundary to joinn-gpu or joinn-shell-desktop",
                        path.join(" -> ")
                    ));
                    continue;
                }
                let mut next: Vec<&str> = edges.get(last).cloned().unwrap_or_default();
                next.sort_unstable();
                for dep in next.into_iter().rev() {
                    let mut longer = path.clone();
                    longer.push(dep);
                    stack.push(longer);
                }
            }
        }
        for dep in &m.deps {
            if let Some((ext, users)) = EXTERNAL.iter().find(|(e, _)| e == dep) {
                if !users.contains(&name) {
                    hits.push(format!(
                        "layers: {ext} in {name}; acceptance is {ext} only in {}",
                        users.join(", ")
                    ));
                }
            }
        }
        if !BELOW.contains(&name) && !m.lints_workspace {
            hits.push(format!(
                "layers: {name} has no [lints] workspace = true; acceptance is the workspace lints table (only joinn-gpu and joinn-shell-desktop hold their own)"
            ));
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::super::Manifest;
    use super::check_layers;

    fn manifest(name: &str, deps: &[&str], lints_workspace: bool) -> Manifest {
        Manifest {
            name: name.into(),
            deps: deps.iter().map(|d| (*d).to_owned()).collect(),
            lints_workspace,
        }
    }

    #[test]
    fn a_reach_through_a_middle_crate_is_refused_with_its_path() {
        let hits = check_layers(&[
            manifest("joinn-cli", &["joinn-visual"], true),
            manifest("joinn-visual", &["joinn-gpu"], true),
            manifest("joinn-gpu", &["joinn-visual", "wgpu"], false),
        ]);
        assert!(
            hits.iter().any(|h| h.starts_with(
                "layers: joinn-cli -> joinn-visual -> joinn-gpu: a crate above the renderer boundary reaches joinn-gpu"
            )),
            "{hits:?}"
        );
    }

    #[test]
    fn wgpu_above_the_boundary_and_a_missing_lints_table_are_refused() {
        let hits = check_layers(&[manifest("joinn-visual", &["joinn-frame", "wgpu"], false)]);
        assert!(
            hits.iter()
                .any(|h| h.starts_with("layers: wgpu in joinn-visual;")),
            "{hits:?}"
        );
        assert!(
            hits.iter()
                .any(|h| h.starts_with("layers: joinn-visual has no [lints] workspace = true")),
            "{hits:?}"
        );
    }

    #[test]
    fn the_table_itself_is_accepted() {
        let hits = check_layers(&[
            manifest(
                "joinn-visual",
                &[
                    "joinn-frame",
                    "joinn-dna",
                    "joinn-link",
                    "joinn-host",
                    "joinn-live",
                ],
                true,
            ),
            manifest(
                "joinn-gpu",
                &[
                    "joinn-frame",
                    "joinn-visual",
                    "wgpu",
                    "bytemuck",
                    "pollster",
                ],
                false,
            ),
            manifest(
                "joinn-shell-desktop",
                &["joinn-visual", "joinn-gpu", "wgpu", "winit", "pollster"],
                false,
            ),
            manifest("xtask", &["joinn-visual", "joinn-gpu"], true),
        ]);
        assert!(hits.is_empty(), "{hits:?}");
    }
}
